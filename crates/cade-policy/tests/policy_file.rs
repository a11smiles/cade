//! A policy file as admins write it, through parsing, serializing and deciding.

use cade_policy::{Action, Decision, Policy};

const CONTOSO: &str = r#"
version: 12
models:
  allow: [sonnet-5.5, opus-5.5]
  byok: false
tools:
  shell: deny
  preapproved: ["terraform plan"]
  network: off
  mcp: [azure-docs]
outputs:
  design_systems: [contoso-ds@4]
  ship_via: pull_request
  iac_lint: required
  pii_classification: required
  strip_visio_macros: false
budgets:
  monthly_tokens: 10000000
telemetry: off
"#;

#[test]
fn policy_serialized_then_parsed_round_trips_unchanged() {
    let p = Policy::from_yaml(CONTOSO).unwrap();
    let yaml = serde_yaml_ng::to_string(&p).unwrap();
    assert_eq!(Policy::from_yaml(&yaml).unwrap(), p);
}

#[test]
fn policy_default_serialized_then_parsed_round_trips_unchanged() {
    let yaml = serde_yaml_ng::to_string(&Policy::default()).unwrap();
    assert_eq!(Policy::from_yaml(&yaml).unwrap(), Policy::default());
}

#[test]
fn policy_from_file_denies_every_governed_action_it_restricts() {
    let p = Policy::from_yaml(CONTOSO).unwrap();
    let denied = [
        (Action::UseModel("gpt-5"), "models.allow"),
        (Action::UseByok, "models.byok = false"),
        (Action::RunShell("terraform apply"), "tools.shell = deny"),
        (Action::AgentNetwork, "tools.network = off"),
        (Action::UseMcpServer("github"), "tools.mcp"),
        (Action::UseDesignSystem("fabrikam-ds@1"), "outputs.design_systems"),
        (Action::ShipByPush, "outputs.ship_via = pull_request"),
        (Action::SendTelemetry, "telemetry = off"),
        (Action::SpendTokens { used: 10_000_000, next: 1 }, "budgets.monthly_tokens"),
    ];
    for (action, rule) in denied {
        assert_eq!(p.decide(&action), Decision::Deny { rule: rule.into() }, "{action:?}");
    }
    assert!(!p.outputs.strip_visio_macros);
    assert_eq!(p.decide(&Action::RunShell("terraform plan -out tf.plan")), Decision::Allow);
}
