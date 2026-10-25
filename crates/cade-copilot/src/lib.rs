//! Copilot for CADE.
//!
//! Both ways of reaching a model go through the GitHub Copilot SDK:
//!
//! - **Copilot**: the user's Copilot seat, via the Copilot CLI runtime the
//!   SDK manages.
//! - **BYOK**: Anthropic, OpenAI, Azure OpenAI or any OpenAI-compatible
//!   endpoint, configured as an SDK `ProviderConfig`. Keys come from the OS
//!   keychain and are never written to disk by CADE.
//!
//! Every tool call Copilot makes is checked against the organization policy
//! before anything runs ([`PolicyPermissions`]). Copilot's edits to a design
//! arrive as a [`ChangeSet`] that the user applies, partly applies or
//! discards. Usage is metered in tokens ([`TokenMeter`]).
//!
//! Backlog: COPILOT epic.

use async_trait::async_trait;
use cade_policy::{Action, Decision, Policy};
use github_copilot_sdk::handler::{PermissionHandler, PermissionResult};
use github_copilot_sdk::{PermissionRequestData, PermissionRequestKind, ProviderConfig, RequestId, SessionId};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

/// Where model calls go.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "provider", rename_all = "kebab-case")]
pub enum ModelSource {
    /// The signed-in user's Copilot seat.
    Copilot {
        model: String,
    },
    Anthropic {
        model: String,
        base_url: Option<String>,
    },
    OpenAi {
        model: String,
        base_url: Option<String>,
    },
    AzureOpenAi {
        deployment: String,
        endpoint: String,
        api_version: String,
    },
    /// Any OpenAI-compatible endpoint (including local runtimes).
    OpenAiCompatible {
        model: String,
        base_url: String,
    },
}

impl ModelSource {
    pub fn is_byok(&self) -> bool {
        !matches!(self, ModelSource::Copilot { .. })
    }

    pub fn model(&self) -> &str {
        match self {
            ModelSource::Copilot { model }
            | ModelSource::Anthropic { model, .. }
            | ModelSource::OpenAi { model, .. }
            | ModelSource::OpenAiCompatible { model, .. } => model,
            ModelSource::AzureOpenAi { deployment, .. } => deployment,
        }
    }

    /// SDK provider config for BYOK sources. `api_key` comes from the keychain.
    pub fn provider_config(&self, api_key: Option<&str>) -> Option<ProviderConfig> {
        let with_key = |p: ProviderConfig| match api_key {
            Some(k) => p.with_api_key(k),
            None => p,
        };
        match self {
            ModelSource::Copilot { .. } => None,
            ModelSource::Anthropic { model, base_url } => Some(with_key(
                ProviderConfig::new(base_url.clone().unwrap_or_else(|| "https://api.anthropic.com".into()))
                    .with_provider_type("anthropic")
                    .with_model_provider("anthropic")
                    .with_model_id(model.clone()),
            )),
            ModelSource::OpenAi { model, base_url } => Some(with_key(
                ProviderConfig::new(base_url.clone().unwrap_or_else(|| "https://api.openai.com/v1".into()))
                    .with_provider_type("openai")
                    .with_model_provider("openai")
                    .with_model_id(model.clone()),
            )),
            ModelSource::AzureOpenAi { deployment, endpoint, .. } => Some(with_key(
                ProviderConfig::new(endpoint.clone())
                    .with_provider_type("azure")
                    .with_model_provider("azure_openai")
                    .with_model_id(deployment.clone()),
            )),
            ModelSource::OpenAiCompatible { model, base_url } => Some(with_key(
                ProviderConfig::new(base_url.clone())
                    .with_provider_type("openai")
                    .with_model_id(model.clone()),
            )),
        }
    }
}

/// Check a model source against policy before starting a session.
pub fn check_source(policy: &Policy, source: &ModelSource) -> Decision {
    if source.is_byok() {
        let d = policy.decide(&Action::UseByok);
        if d != Decision::Allow {
            return d;
        }
    }
    policy.decide(&Action::UseModel(source.model()))
}

/// What the UI should do with a permission request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Prompt {
    /// Show the amber permission prompt with this command or target.
    Ask { summary: String },
}

/// Bridges the user's answer for `Ask` decisions back to the SDK.
#[async_trait]
pub trait PromptUser: Send + Sync + 'static {
    async fn ask(&self, prompt: Prompt) -> bool;
}

/// Permission handler that enforces policy first, then asks the user.
pub struct PolicyPermissions<P: PromptUser> {
    pub policy: Arc<RwLock<Policy>>,
    pub prompt: P,
}

fn summary(data: &PermissionRequestData) -> String {
    for key in ["command", "fullCommandText", "path", "url", "toolName", "serverName"] {
        if let Some(v) = find_str(&data.extra, key) {
            return v;
        }
    }
    format!("{:?}", data.kind)
}

fn find_str(v: &serde_json::Value, key: &str) -> Option<String> {
    match v {
        serde_json::Value::Object(m) => {
            if let Some(serde_json::Value::String(s)) = m.get(key) {
                return Some(s.clone());
            }
            m.values().find_map(|x| find_str(x, key))
        }
        serde_json::Value::Array(a) => a.iter().find_map(|x| find_str(x, key)),
        _ => None,
    }
}

/// Map an SDK permission request to a policy decision.
pub fn decide(policy: &Policy, data: &PermissionRequestData) -> Decision {
    let s = summary(data);
    match data.kind {
        Some(PermissionRequestKind::Shell) => policy.decide(&Action::RunShell(&s)),
        Some(PermissionRequestKind::Url) => policy.decide(&Action::AgentNetwork),
        Some(PermissionRequestKind::Mcp) => policy.decide(&Action::UseMcpServer(&s)),
        // File access is limited to the project folder by the session's cwd; reads are allowed.
        Some(PermissionRequestKind::Read) => Decision::Allow,
        _ => Decision::Ask,
    }
}

#[async_trait]
impl<P: PromptUser> PermissionHandler for PolicyPermissions<P> {
    async fn handle(&self, _session: SessionId, _request: RequestId, data: PermissionRequestData) -> PermissionResult {
        if data.managed_approval_required == Some(true) {
            let ok = self.prompt.ask(Prompt::Ask { summary: summary(&data) }).await;
            return if ok {
                PermissionResult::approve_once()
            } else {
                PermissionResult::reject(Some("Declined by the user".to_string()))
            };
        }
        let decision = match self.policy.read() {
            Ok(p) => decide(&p, &data),
            Err(_) => Decision::Ask,
        };
        match decision {
            Decision::Allow => PermissionResult::approve_once(),
            Decision::Deny { rule } => PermissionResult::reject(Some(format!("Blocked by organization policy ({rule})"))),
            Decision::Ask => {
                if self.prompt.ask(Prompt::Ask { summary: summary(&data) }).await {
                    PermissionResult::approve_once()
                } else {
                    PermissionResult::reject(Some("Declined by the user".to_string()))
                }
            }
        }
    }
}

/// One staged change Copilot proposes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Change {
    pub op: ChangeOp,
    /// Plain words, e.g. "Date of incident → state=error".
    pub description: String,
    /// Where it applies, e.g. "2 frames".
    pub scope: String,
    /// The document operations to apply (CORE operations API, serialized).
    pub ops: Vec<serde_json::Value>,
    pub keep: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ChangeOp {
    Add,
    Mod,
    Del,
}

/// All changes from one Copilot run. Nothing touches the document until applied.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ChangeSet {
    pub run_id: String,
    pub prompt: String,
    pub changes: Vec<Change>,
}

impl ChangeSet {
    pub fn kept(&self) -> impl Iterator<Item = &Change> {
        self.changes.iter().filter(|c| c.keep)
    }
    pub fn toggle(&mut self, index: usize) {
        if let Some(c) = self.changes.get_mut(index) {
            c.keep = !c.keep;
        }
    }
}

/// Token usage for runs, threads and the month.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TokenMeter {
    pub input: u64,
    pub output: u64,
    pub cached: u64,
}

impl TokenMeter {
    pub fn add(&mut self, input: u64, output: u64, cached: u64) {
        self.input += input;
        self.output += output;
        self.cached += cached;
    }
    pub fn total(&self) -> u64 {
        self.input + self.output + self.cached
    }
    /// "186K", "4.2M".
    pub fn short(n: u64) -> String {
        match n {
            n if n >= 1_000_000 => format!("{:.1}M", n as f64 / 1e6),
            n if n >= 1_000 => format!("{}K", (n as f64 / 1e3).round() as u64),
            n => n.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data(kind: PermissionRequestKind, extra: serde_json::Value) -> PermissionRequestData {
        serde_json::from_value(serde_json::json!({ "kind": kind_str(kind), "permissionRequest": extra })).unwrap()
    }
    fn kind_str(k: PermissionRequestKind) -> &'static str {
        match k {
            PermissionRequestKind::Shell => "shell",
            PermissionRequestKind::Url => "url",
            PermissionRequestKind::Mcp => "mcp",
            _ => "read",
        }
    }

    fn strict() -> Policy {
        Policy::from_yaml(
            "version: 12\nmodels: { allow: [sonnet-5.5], byok: false }\ntools: { shell: ask, preapproved: [cade-render], network: off }\n",
        )
        .unwrap()
    }

    #[test]
    fn shell_commands_follow_policy() {
        let p = strict();
        assert_eq!(
            decide(
                &p,
                &data(
                    PermissionRequestKind::Shell,
                    serde_json::json!({"command": "cade-render --check a11y"})
                )
            ),
            Decision::Allow
        );
        assert_eq!(
            decide(
                &p,
                &data(
                    PermissionRequestKind::Shell,
                    serde_json::json!({"command": "npm install chart-lib@4"})
                )
            ),
            Decision::Ask
        );
        assert!(matches!(
            decide(&p, &data(PermissionRequestKind::Url, serde_json::json!({"url": "https://x"}))),
            Decision::Deny { .. }
        ));
    }

    #[test]
    fn byok_is_blocked_when_policy_says_so() {
        let p = strict();
        assert!(matches!(
            check_source(
                &p,
                &ModelSource::Anthropic {
                    model: "claude-sonnet-5-5".into(),
                    base_url: None
                }
            ),
            Decision::Deny { .. }
        ));
        assert_eq!(
            check_source(
                &p,
                &ModelSource::Copilot {
                    model: "sonnet-5.5".into()
                }
            ),
            Decision::Allow
        );
        assert!(
            ModelSource::OpenAi {
                model: "m".into(),
                base_url: None
            }
            .provider_config(Some("k"))
            .is_some()
        );
    }

    #[test]
    fn meters_tokens() {
        let mut m = TokenMeter::default();
        m.add(150_000, 30_000, 6_000);
        assert_eq!(TokenMeter::short(m.total()), "186K");
        assert_eq!(TokenMeter::short(4_200_000), "4.2M");
    }

    #[test]
    fn change_sets_apply_only_kept_changes() {
        let mut cs = ChangeSet {
            run_id: "r7".into(),
            prompt: "Add error states".into(),
            changes: vec![],
        };
        for d in ["Alert", "Date field", "Helper text"] {
            cs.changes.push(Change {
                op: ChangeOp::Add,
                description: d.into(),
                scope: "2 frames".into(),
                ops: vec![],
                keep: true,
            });
        }
        cs.toggle(1);
        assert_eq!(cs.kept().count(), 2);
    }
}
