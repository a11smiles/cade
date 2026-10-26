//! Offline structural checks using the vendored, third-party SchemaStore schema.
//! Hosted GitHub acceptance and CADE answer semantics are separate checks.

use cade_feedback::FeedbackKind;
use serde_json::{Value, json};
use std::path::Path;

fn validator() -> jsonschema::Validator {
    let schema: Value = serde_json::from_str(include_str!("fixtures/github-issue-forms.json")).expect("vendored schema is JSON");
    jsonschema::options()
        .offline()
        .build(&schema)
        .expect("vendored schema compiles without retrieving references")
}

#[test]
fn validate_hosted_forms_accepts_all_three_kinds() {
    let validator = validator();
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.github/ISSUE_TEMPLATE");
    for kind in [FeedbackKind::Bug, FeedbackKind::FeatureRequest, FeedbackKind::Comment] {
        let yaml = std::fs::read_to_string(root.join(kind.template())).unwrap();
        let form: Value = serde_yaml_ng::from_str(&yaml).unwrap();
        assert!(
            validator.validate(&form).is_ok(),
            "{} must satisfy the vendored schema",
            kind.template()
        );
    }
}

#[test]
fn validate_fixture_controls_accepts_valid_and_rejects_invalid_forms() {
    let validator = validator();
    let cases: Vec<Value> = serde_json::from_str(include_str!("fixtures/form-schema-cases.json")).unwrap();
    assert_eq!(cases.len(), 7, "keep every positive and negative fixture");
    for case in cases {
        let valid = case["valid"].as_bool().expect("fixture verdict");
        let form = &case["form"];
        assert_eq!(validator.validate(form).is_ok(), valid, "{}", case["name"]);
        if !valid {
            let path = case["error_path"].as_str().expect("negative fixture identifies its violation");
            assert!(
                validator.iter_errors(form).any(|error| error.instance_path().to_string() == path),
                "{} must reject the intended violation at {path}",
                case["name"]
            );
        }
    }
}

#[test]
fn build_offline_external_references_return_errors() {
    for reference in [
        "https://example.invalid/never-fetch.json",
        "http://example.invalid/never-fetch.json",
        "file:///never-read.json",
    ] {
        assert!(jsonschema::options().offline().build(&json!({"$ref": reference})).is_err());
    }
}
