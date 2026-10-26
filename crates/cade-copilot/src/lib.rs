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
use github_copilot_sdk::{AzureProviderOptions, PermissionRequestData, PermissionRequestKind, ProviderConfig, RequestId, SessionId};
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
            ModelSource::AzureOpenAi {
                deployment,
                endpoint,
                api_version,
            } => Some(with_key(
                ProviderConfig::new(endpoint.clone())
                    .with_provider_type("azure")
                    .with_model_provider("azure_openai")
                    .with_model_id(deployment.clone())
                    .with_azure(AzureProviderOptions {
                        api_version: Some(api_version.clone()),
                    }),
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
        // The allowlist names servers; a tool name must never stand in for one.
        Some(PermissionRequestKind::Mcp) => match find_str(&data.extra, "serverName") {
            Some(server) => policy.decide(&Action::UseMcpServer(&server)),
            None => Decision::Deny { rule: "tools.mcp".into() },
        },
        // File access is limited to the project folder by the session's cwd; reads are allowed.
        Some(PermissionRequestKind::Read) => Decision::Allow,
        _ => Decision::Ask,
    }
}

#[async_trait]
impl<P: PromptUser> PermissionHandler for PolicyPermissions<P> {
    async fn handle(&self, _session: SessionId, _request: RequestId, data: PermissionRequestData) -> PermissionResult {
        let decision = match self.policy.read() {
            Ok(p) => decide(&p, &data),
            Err(_) => return PermissionResult::reject(Some("Blocked because organization policy is unavailable".to_string())),
        };
        // Policy always comes first: managed approval can add a prompt, never lift a deny.
        let decision = match decision {
            Decision::Allow if data.managed_approval_required == Some(true) => Decision::Ask,
            d => d,
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
        let thousands = (n as f64 / 1e3).round() as u64;
        match n {
            // Choose the unit after rounding, so 999,500 reads "1.0M", not "1000K".
            n if n >= 1_000_000 || thousands >= 1_000 => format!("{:.1}M", n as f64 / 1e6),
            n if n >= 1_000 => format!("{thousands}K"),
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

    use std::sync::Mutex;

    fn anthropic() -> ModelSource {
        ModelSource::Anthropic {
            model: "claude-sonnet-5-5".into(),
            base_url: None,
        }
    }

    fn azure() -> ModelSource {
        ModelSource::AzureOpenAi {
            deployment: "gpt-5-prod".into(),
            endpoint: "https://contoso.openai.azure.com".into(),
            api_version: "2026-06-01".into(),
        }
    }

    fn compatible() -> ModelSource {
        ModelSource::OpenAiCompatible {
            model: "llama-4".into(),
            base_url: "http://localhost:11434/v1".into(),
        }
    }

    fn raw(value: serde_json::Value) -> PermissionRequestData {
        serde_json::from_value(value).unwrap()
    }

    #[test]
    fn is_byok_each_source_reports_whether_it_uses_a_users_key() {
        assert!(!ModelSource::Copilot { model: "auto".into() }.is_byok());
        assert!(anthropic().is_byok());
        assert!(
            ModelSource::OpenAi {
                model: "gpt-5".into(),
                base_url: None
            }
            .is_byok()
        );
        assert!(azure().is_byok());
        assert!(compatible().is_byok());
    }

    #[test]
    fn model_azure_source_returns_deployment_name() {
        assert_eq!(azure().model(), "gpt-5-prod");
        assert_eq!(compatible().model(), "llama-4");
    }

    #[test]
    fn provider_config_copilot_source_returns_none() {
        assert!(ModelSource::Copilot { model: "auto".into() }.provider_config(Some("k")).is_none());
    }

    #[test]
    fn provider_config_anthropic_without_base_url_uses_public_endpoint_and_key() {
        let c = anthropic().provider_config(Some("sk-ant-test")).unwrap();
        assert_eq!(c.base_url, "https://api.anthropic.com");
        assert_eq!(c.provider_type.as_deref(), Some("anthropic"));
        assert_eq!(c.model_provider.as_deref(), Some("anthropic"));
        assert_eq!(c.model_id.as_deref(), Some("claude-sonnet-5-5"));
        assert_eq!(c.api_key.as_deref(), Some("sk-ant-test"));
    }

    #[test]
    fn provider_config_openai_custom_base_url_overrides_default() {
        let s = ModelSource::OpenAi {
            model: "gpt-5".into(),
            base_url: Some("https://proxy.contoso.com/v1".into()),
        };
        let c = s.provider_config(None).unwrap();
        assert_eq!(c.base_url, "https://proxy.contoso.com/v1");
        assert_eq!(c.provider_type.as_deref(), Some("openai"));
        assert_eq!(c.model_provider.as_deref(), Some("openai"));
        assert_eq!(c.model_id.as_deref(), Some("gpt-5"));
        assert_eq!(c.api_key, None);
    }

    #[test]
    fn provider_config_openai_without_base_url_uses_public_endpoint() {
        let s = ModelSource::OpenAi {
            model: "gpt-5".into(),
            base_url: None,
        };
        assert_eq!(s.provider_config(None).unwrap().base_url, "https://api.openai.com/v1");
    }

    #[test]
    fn provider_config_azure_source_targets_endpoint_and_deployment() {
        let c = azure().provider_config(Some("azure-key")).unwrap();
        assert_eq!(c.base_url, "https://contoso.openai.azure.com");
        assert_eq!(c.provider_type.as_deref(), Some("azure"));
        assert_eq!(c.model_provider.as_deref(), Some("azure_openai"));
        assert_eq!(c.model_id.as_deref(), Some("gpt-5-prod"));
        assert_eq!(c.api_key.as_deref(), Some("azure-key"));
    }

    #[test]
    fn provider_config_azure_source_passes_api_version() {
        let c = azure().provider_config(None).unwrap();
        assert_eq!(c.azure.and_then(|a| a.api_version).as_deref(), Some("2026-06-01"));
    }

    #[test]
    fn provider_config_non_azure_sources_set_no_azure_options() {
        for source in [anthropic(), compatible()] {
            assert!(source.provider_config(None).unwrap().azure.is_none(), "{source:?}");
        }
    }

    #[test]
    fn provider_config_openai_compatible_source_uses_openai_wire_without_provider_name() {
        let c = compatible().provider_config(None).unwrap();
        assert_eq!(c.base_url, "http://localhost:11434/v1");
        assert_eq!(c.provider_type.as_deref(), Some("openai"));
        assert_eq!(c.model_provider, None);
        assert_eq!(c.model_id.as_deref(), Some("llama-4"));
        assert_eq!(c.api_key, None);
    }

    #[test]
    fn check_source_byok_allowed_but_model_unlisted_denies_with_models_allow_rule() {
        let p = Policy::from_yaml("version: 1\nmodels: { allow: [gpt-5] }\n").unwrap();
        assert_eq!(
            check_source(&p, &anthropic()),
            Decision::Deny {
                rule: "models.allow".into()
            }
        );
    }

    #[test]
    fn check_source_byok_disabled_denies_before_checking_model() {
        let p = Policy::from_yaml("version: 1\nmodels: { allow: [claude-sonnet-5-5], byok: false }\n").unwrap();
        assert_eq!(
            check_source(&p, &anthropic()),
            Decision::Deny {
                rule: "models.byok = false".into()
            }
        );
    }

    #[test]
    fn check_source_copilot_source_ignores_byok_switch() {
        let p = Policy::from_yaml("version: 1\nmodels: { byok: false }\n").unwrap();
        assert_eq!(check_source(&p, &ModelSource::Copilot { model: "auto".into() }), Decision::Allow);
    }

    #[test]
    fn decide_read_request_allows_without_policy_check() {
        let p = Policy::from_yaml("version: 1\ntools: { shell: deny, network: off }\n").unwrap();
        assert_eq!(
            decide(&p, &raw(serde_json::json!({"kind": "read", "path": "a.cade"}))),
            Decision::Allow
        );
    }

    #[test]
    fn decide_write_and_unknown_kinds_ask_the_user() {
        let p = Policy::from_yaml("version: 1\ntools: { shell: allow }\n").unwrap();
        for kind in ["write", "custom-tool", "memory", "hook", "something-new"] {
            assert_eq!(decide(&p, &raw(serde_json::json!({"kind": kind}))), Decision::Ask, "{kind}");
        }
        assert_eq!(decide(&p, &raw(serde_json::json!({}))), Decision::Ask);
    }

    #[test]
    fn decide_url_request_with_network_on_allows() {
        assert_eq!(
            decide(
                &Policy::default(),
                &raw(serde_json::json!({"kind": "url", "url": "https://learn.microsoft.com"}))
            ),
            Decision::Allow
        );
    }

    #[test]
    fn decide_mcp_request_checks_server_name_against_allowlist() {
        let p = Policy::from_yaml("version: 1\ntools: { mcp: [azure-docs] }\n").unwrap();
        let ok = raw(serde_json::json!({"kind": "mcp", "permissionRequest": {"serverName": "azure-docs"}}));
        let other = raw(serde_json::json!({"kind": "mcp", "permissionRequest": {"serverName": "github"}}));
        assert_eq!(decide(&p, &ok), Decision::Allow);
        assert_eq!(decide(&p, &other), Decision::Deny { rule: "tools.mcp".into() });
    }

    #[test]
    fn decide_mcp_request_tool_named_like_allowed_server_denies() {
        let p = Policy::from_yaml("version: 1\ntools: { mcp: [azure-docs] }\n").unwrap();
        let d = raw(serde_json::json!({"kind": "mcp", "serverName": "evil", "toolName": "azure-docs"}));
        assert_eq!(decide(&p, &d), Decision::Deny { rule: "tools.mcp".into() });
    }

    #[test]
    fn decide_mcp_request_allowed_server_with_other_tool_name_allows() {
        let p = Policy::from_yaml("version: 1\ntools: { mcp: [azure-docs] }\n").unwrap();
        let d = raw(serde_json::json!({"kind": "mcp", "serverName": "azure-docs", "toolName": "search"}));
        assert_eq!(decide(&p, &d), Decision::Allow);
    }

    #[test]
    fn decide_mcp_request_without_server_name_denies() {
        let listed = Policy::from_yaml("version: 1\ntools: { mcp: [azure-docs] }\n").unwrap();
        let d = raw(serde_json::json!({"kind": "mcp", "toolName": "azure-docs"}));
        assert_eq!(decide(&listed, &d), Decision::Deny { rule: "tools.mcp".into() });
        assert_eq!(decide(&Policy::default(), &d), Decision::Deny { rule: "tools.mcp".into() });
    }

    #[test]
    fn decide_shell_request_with_full_command_text_uses_it() {
        let p = Policy::from_yaml("version: 1\ntools: { shell: deny, preapproved: [bicep] }\n").unwrap();
        let d = raw(serde_json::json!({"kind": "shell", "permissionRequest": {"fullCommandText": "bicep build main.bicep"}}));
        assert_eq!(decide(&p, &d), Decision::Allow);
    }

    #[test]
    fn summary_command_key_wins_over_other_keys() {
        let d = raw(serde_json::json!({"kind": "shell", "path": "/p", "nested": {"command": "ls -la"}}));
        assert_eq!(summary(&d), "ls -la");
    }

    #[test]
    fn summary_no_known_key_falls_back_to_kind() {
        assert_eq!(summary(&raw(serde_json::json!({"kind": "write"}))), "Some(Write)");
    }

    #[test]
    fn find_str_value_inside_array_is_found() {
        let v = serde_json::json!({"a": [1, {"b": {"url": "https://x.test"}}]});
        assert_eq!(find_str(&v, "url").as_deref(), Some("https://x.test"));
        assert_eq!(find_str(&v, "path"), None);
        assert_eq!(find_str(&serde_json::json!({"url": 3}), "url"), None);
    }

    /// Records every prompt and answers with a fixed choice.
    #[derive(Clone)]
    struct FakeUser {
        answer: bool,
        asked: Arc<Mutex<Vec<Prompt>>>,
    }

    impl FakeUser {
        fn new(answer: bool) -> Self {
            FakeUser {
                answer,
                asked: Arc::default(),
            }
        }
        fn asked(&self) -> Vec<Prompt> {
            self.asked.lock().unwrap().clone()
        }
    }

    #[async_trait]
    impl PromptUser for FakeUser {
        async fn ask(&self, prompt: Prompt) -> bool {
            self.asked.lock().unwrap().push(prompt);
            self.answer
        }
    }

    fn handler(yaml: &str, user: &FakeUser) -> PolicyPermissions<FakeUser> {
        PolicyPermissions {
            policy: Arc::new(RwLock::new(Policy::from_yaml(yaml).unwrap())),
            prompt: user.clone(),
        }
    }

    /// "approve" or "reject: <feedback>"; the SDK's decision type isn't public.
    fn outcome(r: PermissionResult) -> String {
        match r {
            PermissionResult::Decision { decision, .. } => {
                let d = format!("{decision:?}");
                if d.starts_with("ApproveOnce") {
                    "approve".into()
                } else if d.starts_with("Reject") {
                    let fb = d.split('"').nth(1).unwrap_or_default();
                    format!("reject: {fb}")
                } else {
                    d
                }
            }
            PermissionResult::NoResult => "no result".into(),
        }
    }

    async fn run(h: &PolicyPermissions<FakeUser>, data: serde_json::Value) -> String {
        outcome(h.handle(SessionId::new("s"), RequestId::new("r"), raw(data)).await)
    }

    #[tokio::test]
    async fn handle_policy_deny_rejects_with_rule_and_never_prompts() {
        let user = FakeUser::new(true);
        let h = handler("version: 1\ntools: { shell: allow, network: off }\n", &user);
        let r = run(&h, serde_json::json!({"kind": "url", "url": "https://exfil.test"})).await;
        assert_eq!(r, "reject: Blocked by organization policy (tools.network = off)");
        assert_eq!(user.asked(), vec![]);
    }

    #[tokio::test]
    async fn handle_policy_allow_approves_without_prompting() {
        let user = FakeUser::new(false);
        let h = handler("version: 1\ntools: { shell: deny, preapproved: [cade-render] }\n", &user);
        let r = run(&h, serde_json::json!({"kind": "shell", "command": "cade-render --check"})).await;
        assert_eq!(r, "approve");
        assert_eq!(user.asked(), vec![]);
    }

    #[tokio::test]
    async fn handle_policy_ask_user_accepts_approves_after_prompting_with_command() {
        let user = FakeUser::new(true);
        let h = handler("version: 1\n", &user);
        let r = run(&h, serde_json::json!({"kind": "shell", "command": "npm install chart-lib@4"})).await;
        assert_eq!(r, "approve");
        assert_eq!(
            user.asked(),
            vec![Prompt::Ask {
                summary: "npm install chart-lib@4".into()
            }]
        );
    }

    #[tokio::test]
    async fn handle_policy_ask_user_declines_rejects_as_declined() {
        let user = FakeUser::new(false);
        let h = handler("version: 1\n", &user);
        let r = run(&h, serde_json::json!({"kind": "write", "path": "src/App.tsx"})).await;
        assert_eq!(r, "reject: Declined by the user");
        assert_eq!(
            user.asked(),
            vec![Prompt::Ask {
                summary: "src/App.tsx".into()
            }]
        );
    }

    #[tokio::test]
    async fn handle_managed_approval_required_prompts_even_when_policy_allows() {
        let user = FakeUser::new(false);
        let h = handler("version: 1\ntools: { shell: allow }\n", &user);
        let r = run(
            &h,
            serde_json::json!({"kind": "shell", "command": "ls", "managedApprovalRequired": true}),
        )
        .await;
        assert_eq!(r, "reject: Declined by the user");
        assert_eq!(user.asked(), vec![Prompt::Ask { summary: "ls".into() }]);
    }

    #[tokio::test]
    async fn handle_managed_approval_required_policy_deny_rejects_without_prompting() {
        let user = FakeUser::new(true);
        let h = handler("version: 1\ntools: { shell: deny }\n", &user);
        let r = run(
            &h,
            serde_json::json!({"kind": "shell", "command": "rm -rf build", "managedApprovalRequired": true}),
        )
        .await;
        assert_eq!(r, "reject: Blocked by organization policy (tools.shell = deny)");
        assert_eq!(user.asked(), vec![]);
    }

    #[tokio::test]
    async fn handle_managed_approval_required_policy_ask_prompts_once() {
        let user = FakeUser::new(true);
        let h = handler("version: 1\n", &user);
        let r = run(
            &h,
            serde_json::json!({"kind": "shell", "command": "npm test", "managedApprovalRequired": true}),
        )
        .await;
        assert_eq!(r, "approve");
        assert_eq!(
            user.asked(),
            vec![Prompt::Ask {
                summary: "npm test".into()
            }]
        );
    }

    #[tokio::test]
    async fn handle_poisoned_allow_policy_rejects_without_prompting() {
        // Unavailable policy must fail closed, not fall back to Ask: user approval
        // cannot substitute for trustworthy policy evaluation, even for an allow policy.
        let user = FakeUser::new(true);
        let h = handler("version: 1\ntools: { shell: allow }\n", &user);
        poison_policy_lock(h.policy.clone()).await;
        assert!(h.policy.is_poisoned());
        let r = run(&h, serde_json::json!({"kind": "shell", "command": "ls"})).await;
        assert_eq!(r, "reject: Blocked because organization policy is unavailable");
        assert!(user.asked().is_empty());
        assert!(h.policy.is_poisoned());
    }

    async fn poison_policy_lock(policy: Arc<RwLock<Policy>>) {
        tokio::task::spawn_blocking(move || {
            assert!(
                std::thread::spawn(move || {
                    let _guard = policy.write().unwrap();
                    panic!("synthetic policy lock poisoning");
                })
                .join()
                .is_err()
            );
        })
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn handle_poisoned_deny_policy_managed_approval_rejects_without_prompting() {
        let user = FakeUser::new(true);
        let h = handler("version: 1\ntools: { shell: deny }\n", &user);
        poison_policy_lock(h.policy.clone()).await;
        assert!(h.policy.is_poisoned());

        let r = run(
            &h,
            serde_json::json!({"kind": "shell", "command": "ls", "managedApprovalRequired": true}),
        )
        .await;

        assert!(
            r.starts_with("reject: ") && user.asked().is_empty() && h.policy.is_poisoned(),
            "unavailable policy must reject without prompting or clearing poison; outcome={r}, prompts={}, poisoned={}",
            user.asked().len(),
            h.policy.is_poisoned()
        );
    }

    #[tokio::test]
    async fn handle_poisoned_deny_policy_without_managed_approval_rejects_without_prompting() {
        let user = FakeUser::new(true);
        let h = handler("version: 1\ntools: { shell: deny }\n", &user);
        poison_policy_lock(h.policy.clone()).await;
        assert!(h.policy.is_poisoned());

        let r = run(&h, serde_json::json!({"kind": "shell", "command": "ls"})).await;

        assert!(
            r.starts_with("reject: ") && user.asked().is_empty() && h.policy.is_poisoned(),
            "unavailable policy must reject without prompting or clearing poison; outcome={r}, prompts={}, poisoned={}",
            user.asked().len(),
            h.policy.is_poisoned()
        );
    }

    fn change(description: &str, keep: bool) -> Change {
        Change {
            op: ChangeOp::Mod,
            description: description.into(),
            scope: "1 frame".into(),
            ops: vec![],
            keep,
        }
    }

    #[test]
    fn kept_returns_only_kept_changes_in_order() {
        let cs = ChangeSet {
            changes: vec![change("a", true), change("b", false), change("c", true)],
            ..ChangeSet::default()
        };
        let kept: Vec<_> = cs.kept().map(|c| c.description.as_str()).collect();
        assert_eq!(kept, ["a", "c"]);
    }

    #[test]
    fn toggle_twice_restores_and_out_of_range_is_ignored() {
        let mut cs = ChangeSet {
            changes: vec![change("a", true)],
            ..ChangeSet::default()
        };
        cs.toggle(0);
        assert!(!cs.changes[0].keep);
        cs.toggle(0);
        assert!(cs.changes[0].keep);
        cs.toggle(5);
        assert_eq!(cs.changes, vec![change("a", true)]);
    }

    #[test]
    fn change_op_serializes_as_uppercase_badge_text() {
        assert_eq!(
            serde_json::to_value([ChangeOp::Add, ChangeOp::Mod, ChangeOp::Del]).unwrap(),
            serde_json::json!(["ADD", "MOD", "DEL"])
        );
    }

    #[test]
    fn model_source_serializes_with_kebab_case_provider_tag() {
        assert_eq!(
            serde_json::to_value(azure()).unwrap(),
            serde_json::json!({
                "provider": "azure-open-ai",
                "deployment": "gpt-5-prod",
                "endpoint": "https://contoso.openai.azure.com",
                "api_version": "2026-06-01"
            })
        );
        let tags: Vec<_> = [ModelSource::Copilot { model: "m".into() }, anthropic(), compatible()]
            .iter()
            .map(|s| serde_json::to_value(s).unwrap()["provider"].clone())
            .collect();
        assert_eq!(tags, ["copilot", "anthropic", "open-ai-compatible"]);
    }

    #[test]
    fn add_repeated_calls_accumulate_each_counter() {
        let mut m = TokenMeter::default();
        m.add(10, 20, 30);
        m.add(1, 2, 3);
        assert_eq!(
            m,
            TokenMeter {
                input: 11,
                output: 22,
                cached: 33
            }
        );
        assert_eq!(m.total(), 66);
    }

    #[test]
    fn short_each_magnitude_formats_with_its_unit() {
        assert_eq!(TokenMeter::short(0), "0");
        assert_eq!(TokenMeter::short(999), "999");
        assert_eq!(TokenMeter::short(1_000), "1K");
        assert_eq!(TokenMeter::short(1_499), "1K");
        assert_eq!(TokenMeter::short(1_500), "2K");
        assert_eq!(TokenMeter::short(1_000_000), "1.0M");
        assert_eq!(TokenMeter::short(10_000_000), "10.0M");
    }

    #[test]
    fn short_just_under_a_million_formats_as_millions() {
        assert_eq!(TokenMeter::short(999_499), "999K");
        assert_eq!(TokenMeter::short(999_500), "1.0M");
        assert_eq!(TokenMeter::short(999_999), "1.0M");
    }
}
