//! Organization policy for CADE.
//!
//! Admins keep policy as code in `<org>/.github` at `cade/policy.yml`.
//! Clients fetch it on sign-in (POLICY epic), cache it, and ask this crate
//! for a [`Decision`] before every governed action. A missing policy means
//! [`Policy::default`], which is permissive for personal use.

use serde::{Deserialize, Serialize};

pub const POLICY_PATH: &str = "cade/policy.yml";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Policy {
    pub version: u32,
    #[serde(default)]
    pub models: Models,
    #[serde(default)]
    pub tools: Tools,
    #[serde(default)]
    pub outputs: Outputs,
    #[serde(default)]
    pub budgets: Budgets,
    /// Off unless an admin turns it on.
    #[serde(default = "off")]
    pub telemetry: Switch,
}

impl Default for Policy {
    fn default() -> Self {
        Policy {
            version: 0,
            models: Models::default(),
            tools: Tools::default(),
            outputs: Outputs::default(),
            budgets: Budgets::default(),
            telemetry: Switch::Off,
        }
    }
}

fn on() -> Switch {
    Switch::On
}
fn off() -> Switch {
    Switch::Off
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Switch {
    On,
    Off,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolMode {
    Allow,
    Ask,
    Deny,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Models {
    /// Empty means every model Copilot offers.
    #[serde(default)]
    pub allow: Vec<String>,
    #[serde(default = "yes")]
    pub byok: bool,
}

fn yes() -> bool {
    true
}

impl Default for Models {
    fn default() -> Self {
        Models { allow: vec![], byok: true }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tools {
    #[serde(default = "ask")]
    pub shell: ToolMode,
    /// Command prefixes that skip the prompt, e.g. `terraform plan`.
    #[serde(default)]
    pub preapproved: Vec<String>,
    #[serde(default = "on")]
    pub network: Switch,
    /// `None` means any MCP server; `Some(list)` is an allowlist.
    #[serde(default)]
    pub mcp: Option<Vec<String>>,
}

fn ask() -> ToolMode {
    ToolMode::Ask
}

impl Default for Tools {
    fn default() -> Self {
        Tools {
            shell: ToolMode::Ask,
            preapproved: vec![],
            network: Switch::On,
            mcp: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShipVia {
    PullRequest,
    Push,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Requirement {
    Required,
    Optional,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Outputs {
    /// Empty means any design system. Entries look like `contoso-ds@4`.
    #[serde(default)]
    pub design_systems: Vec<String>,
    #[serde(default = "pr")]
    pub ship_via: ShipVia,
    #[serde(default = "optional")]
    pub iac_lint: Requirement,
    #[serde(default = "optional")]
    pub pii_classification: Requirement,
    #[serde(default = "yes")]
    pub strip_visio_macros: bool,
}

fn pr() -> ShipVia {
    ShipVia::PullRequest
}
fn optional() -> Requirement {
    Requirement::Optional
}

impl Default for Outputs {
    fn default() -> Self {
        Outputs {
            design_systems: vec![],
            ship_via: ShipVia::PullRequest,
            iac_lint: Requirement::Optional,
            pii_classification: Requirement::Optional,
            strip_visio_macros: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Budgets {
    /// Monthly Copilot token budget per user. `None` means no CADE-level cap.
    #[serde(default)]
    pub monthly_tokens: Option<u64>,
}

/// A governed action CADE is about to take.
#[derive(Debug, Clone, PartialEq)]
pub enum Action<'a> {
    UseModel(&'a str),
    UseByok,
    RunShell(&'a str),
    AgentNetwork,
    UseMcpServer(&'a str),
    UseDesignSystem(&'a str),
    ShipByPush,
    SendTelemetry,
    /// Tokens already used this month plus the estimate for the next request.
    SpendTokens {
        used: u64,
        next: u64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    Allow,
    /// Ask the user first (permission prompt).
    Ask,
    /// Blocked. `rule` is shown to the user, e.g. `tools.network = off`.
    Deny {
        rule: String,
    },
}

impl Policy {
    pub fn from_yaml(text: &str) -> Result<Self, PolicyError> {
        let p: Policy = serde_yaml_ng::from_str(text).map_err(|e| PolicyError::Parse(e.to_string()))?;
        Ok(p)
    }

    pub fn decide(&self, action: &Action<'_>) -> Decision {
        let deny = |rule: &str| Decision::Deny { rule: rule.to_string() };
        match action {
            Action::UseModel(m) => {
                if self.models.allow.is_empty() || self.models.allow.iter().any(|a| a.eq_ignore_ascii_case(m)) {
                    Decision::Allow
                } else {
                    deny("models.allow")
                }
            }
            Action::UseByok => {
                if self.models.byok {
                    Decision::Allow
                } else {
                    deny("models.byok = false")
                }
            }
            Action::RunShell(cmd) => {
                let cmd = cmd.trim();
                if self.tools.preapproved.iter().any(|p| cmd == p || cmd.starts_with(&format!("{p} "))) {
                    return Decision::Allow;
                }
                match self.tools.shell {
                    ToolMode::Allow => Decision::Allow,
                    ToolMode::Ask => Decision::Ask,
                    ToolMode::Deny => deny("tools.shell = deny"),
                }
            }
            Action::AgentNetwork => match self.tools.network {
                Switch::On => Decision::Allow,
                Switch::Off => deny("tools.network = off"),
            },
            Action::UseMcpServer(name) => match &self.tools.mcp {
                None => Decision::Allow,
                Some(list) if list.iter().any(|n| n == name) => Decision::Allow,
                Some(_) => deny("tools.mcp"),
            },
            Action::UseDesignSystem(ds) => {
                if self.outputs.design_systems.is_empty()
                    || self
                        .outputs
                        .design_systems
                        .iter()
                        .any(|d| d == ds || ds.starts_with(&format!("{d}.")))
                {
                    Decision::Allow
                } else {
                    deny("outputs.design_systems")
                }
            }
            Action::ShipByPush => match self.outputs.ship_via {
                ShipVia::Push => Decision::Allow,
                ShipVia::PullRequest => deny("outputs.ship_via = pull_request"),
            },
            Action::SendTelemetry => match self.telemetry {
                Switch::On => Decision::Allow,
                Switch::Off => deny("telemetry = off"),
            },
            Action::SpendTokens { used, next } => match self.budgets.monthly_tokens {
                Some(budget) if used + next > budget => deny("budgets.monthly_tokens"),
                _ => Decision::Allow,
            },
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PolicyError {
    #[error("policy.yml is not valid: {0}")]
    Parse(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTOSO: &str = r#"
version: 12
models:
  allow: [sonnet-5.5, opus-5.5, auto]
  byok: false
tools:
  shell: ask
  preapproved: [cade-render, bicep, "terraform plan"]
  network: off
  mcp: [azure-docs]
outputs:
  design_systems: [contoso-ds@4]
  ship_via: pull_request
  iac_lint: required
  pii_classification: required
budgets:
  monthly_tokens: 10000000
telemetry: off
"#;

    #[test]
    fn parses_the_admin_screen_policy() {
        let p = Policy::from_yaml(CONTOSO).unwrap();
        assert_eq!(p.version, 12);
        assert!(!p.models.byok);
        assert_eq!(p.tools.network, Switch::Off);
        assert_eq!(p.outputs.iac_lint, Requirement::Required);
    }

    #[test]
    fn enforces_each_rule() {
        let p = Policy::from_yaml(CONTOSO).unwrap();
        assert_eq!(p.decide(&Action::UseModel("opus-5.5")), Decision::Allow);
        assert!(matches!(p.decide(&Action::UseModel("other")), Decision::Deny { .. }));
        assert!(matches!(p.decide(&Action::UseByok), Decision::Deny { .. }));
        assert_eq!(p.decide(&Action::RunShell("terraform plan -out x")), Decision::Allow);
        assert_eq!(p.decide(&Action::RunShell("npm install chart-lib@4")), Decision::Ask);
        assert_eq!(
            p.decide(&Action::AgentNetwork),
            Decision::Deny {
                rule: "tools.network = off".into()
            }
        );
        assert!(matches!(p.decide(&Action::UseMcpServer("random")), Decision::Deny { .. }));
        assert!(matches!(p.decide(&Action::ShipByPush), Decision::Deny { .. }));
        assert!(matches!(p.decide(&Action::SendTelemetry), Decision::Deny { .. }));
        assert_eq!(
            p.decide(&Action::SpendTokens {
                used: 9_900_000,
                next: 50_000
            }),
            Decision::Allow
        );
        assert!(matches!(
            p.decide(&Action::SpendTokens {
                used: 9_990_000,
                next: 50_000
            }),
            Decision::Deny { .. }
        ));
    }

    #[test]
    fn no_policy_is_permissive_except_telemetry() {
        let p = Policy::default();
        assert_eq!(p.decide(&Action::UseByok), Decision::Allow);
        assert_eq!(p.decide(&Action::AgentNetwork), Decision::Allow);
        assert!(matches!(p.decide(&Action::SendTelemetry), Decision::Deny { .. }));
    }

    #[test]
    fn rejects_unknown_keys() {
        assert!(Policy::from_yaml("version: 1\nmodles: {}\n").is_err());
    }
}
