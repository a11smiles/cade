//! The in-app dialog files the same issues as the forms in
//! `.github/ISSUE_TEMPLATE/`, and its payload matches what `app/ui` sends.

use cade_feedback::{Feedback, FeedbackKind};
use std::path::PathBuf;

const KINDS: [FeedbackKind; 3] = [FeedbackKind::Bug, FeedbackKind::FeatureRequest, FeedbackKind::Comment];

fn repo_file(path: &str) -> serde_yaml_ng::Value {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..").join(path);
    serde_yaml_ng::from_str(&std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("{}: {e}", p.display()))).unwrap()
}

#[test]
fn feedback_kind_labels_and_title_match_each_issue_form() {
    for kind in KINDS {
        let form = repo_file(&format!(".github/ISSUE_TEMPLATE/{}", kind.template()));
        let labels: Vec<&str> = form["labels"].as_sequence().unwrap().iter().map(|v| v.as_str().unwrap()).collect();
        assert_eq!(labels, kind.labels(), "{kind:?}");
        assert_eq!(form["title"].as_str(), Some(kind.title_prefix()), "{kind:?}");
    }
}

#[test]
fn feedback_kind_labels_are_all_defined_in_labels_yml() {
    let defined: Vec<String> = repo_file(".github/labels.yml")
        .as_sequence()
        .unwrap()
        .iter()
        .map(|l| l["name"].as_str().unwrap().to_string())
        .collect();
    for kind in KINDS {
        for label in kind.labels() {
            assert!(defined.iter().any(|d| d == label), "{label} missing from labels.yml");
        }
    }
}

#[test]
fn feedback_from_ui_json_builds_redacted_issue() {
    // Shape sent by app/ui/src/feedback/FeedbackDialog.tsx (types.ts).
    let json = serde_json::json!({
        "kind": "feature-request",
        "title": "Import from /Users/ann/Downloads",
        "answers": [
            {"heading": "What problem would this solve?", "text": "Mail me at ann@contoso.com"},
            {"heading": "What would you like CADE to do?", "text": "Read .fig files"},
            {"heading": "Which output type?", "text": ""}
        ],
        "environment": {"app_version": "0.1.0", "os": null, "gpu": null, "policy_version": null, "logs": null}
    });
    let feedback: Feedback = serde_json::from_value(json).unwrap();
    let issue = feedback.to_issue().unwrap();

    assert_eq!(issue.title, "[Feature] Import from /Users/<user>/Downloads");
    assert_eq!(issue.labels, vec!["enhancement"]);
    assert_eq!(
        issue.body,
        "### What problem would this solve?\n\nMail me at <email>\n\n\
         ### What would you like CADE to do?\n\nRead .fig files\n\n\
         ### Which output type?\n\n_No response_\n\n\
         <details><summary>Environment</summary>\n\n- **CADE version:** 0.1.0\n\n</details>\n\n\
         <sub>Sent from CADE (Help › Send feedback).</sub>\n"
    );
}
