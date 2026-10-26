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
fn feedback_kind_required_headings_match_each_issue_form() {
    for kind in KINDS {
        let form = repo_file(&format!(".github/ISSUE_TEMPLATE/{}", kind.template()));
        let required: Vec<&str> = form["body"]
            .as_sequence()
            .unwrap()
            .iter()
            .filter(|item| item["validations"]["required"].as_bool() == Some(true))
            .map(|item| item["attributes"]["label"].as_str().unwrap())
            .collect();
        assert_eq!(required, kind.required_headings(), "{kind:?}");
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
    assert_eq!(issue.labels, vec!["enhancement", "needs-triage", "type:feature"]);
    assert_eq!(
        issue.body,
        "### What problem would this solve?\n\nMail me at <email>\n\n\
         ### What would you like CADE to do?\n\nRead .fig files\n\n\
         ### Which output type?\n\n_No response_\n\n\
         ### What do you do today instead?\n\n_No response_\n\n\
         ### Who would benefit?\n\n_No response_\n\n\
         ### Would you like to contribute?\n\n_No response_\n\n\
         <details><summary>Environment</summary>\n\n- **CADE version:** 0.1.0\n\n</details>\n\n\
         <sub>Sent from CADE (Help › Send feedback).</sub>\n"
    );
}

#[test]
fn template_bug_name_matches_requested_name() {
    let form = repo_file(".github/ISSUE_TEMPLATE/bug.yml");
    assert_eq!(form["name"].as_str(), Some("Bug report"));
}

#[test]
fn template_comment_name_matches_requested_name() {
    let form = repo_file(".github/ISSUE_TEMPLATE/comment.yml");
    assert_eq!(form["name"].as_str(), Some("Comment or question"));
}

fn assert_form_ids(kind: FeedbackKind, expected: &[&str]) {
    let form = repo_file(&format!(".github/ISSUE_TEMPLATE/{}", kind.template()));
    let mut actual: Vec<&str> = form["body"]
        .as_sequence()
        .unwrap()
        .iter()
        .filter(|item| item["type"].as_str() != Some("markdown"))
        .map(|item| item["id"].as_str().expect("each control needs a payload ID"))
        .collect();
    let mut expected = expected.to_vec();
    actual.sort_unstable();
    expected.sort_unstable();
    assert_eq!(actual, expected, "{kind:?} control IDs");
}

#[test]
fn template_bug_control_ids_match_answer_and_environment_keys() {
    assert_form_ids(
        FeedbackKind::Bug,
        &[
            "what-happened",
            "steps",
            "expected",
            "actual",
            "output",
            "area",
            "severity",
            "app_version",
            "os",
            "gpu",
            "policy_version",
            "screenshot_url",
        ],
    );
}

#[test]
fn template_feature_control_ids_include_all_requested_fields() {
    assert_form_ids(
        FeedbackKind::FeatureRequest,
        &["problem", "proposal", "alternatives", "output", "beneficiaries", "contribute"],
    );
}

#[test]
fn template_comment_control_ids_include_all_requested_fields() {
    assert_form_ids(FeedbackKind::Comment, &["where", "comment", "output", "reply"]);
}

fn form_options(form: &serde_yaml_ng::Value, id: &str) -> Vec<String> {
    let field = form["body"]
        .as_sequence()
        .unwrap()
        .iter()
        .find(|item| item["id"].as_str() == Some(id));
    assert!(field.is_some(), "missing {id} control");
    field.unwrap()["attributes"]["options"]
        .as_sequence()
        .expect("choice controls need an options array")
        .iter()
        .map(|option| option.as_str().expect("options must be strings").to_string())
        .collect()
}

#[test]
fn template_output_options_each_kind_match_exact_requested_choices() {
    for kind in KINDS {
        let form = repo_file(&format!(".github/ISSUE_TEMPLATE/{}", kind.template()));
        assert_eq!(
            form_options(&form, "output"),
            [
                "App",
                "Image",
                "Deck",
                "Document",
                "Motion",
                "Diagram",
                "Architecture",
                "Data model",
                "System",
                "Other"
            ],
            "{kind:?} output choices"
        );
    }
}

#[test]
fn template_feature_beneficiaries_options_match_requested_choices() {
    let form = repo_file(".github/ISSUE_TEMPLATE/feature_request.yml");
    assert_eq!(form_options(&form, "beneficiaries"), ["individual", "team", "admin/security"]);
}

#[test]
fn labels_each_kind_retains_legacy_and_required_sets() {
    for (kind, expected) in [
        (FeedbackKind::Bug, vec!["bug", "needs-triage", "type:bug"]),
        (FeedbackKind::FeatureRequest, vec!["enhancement", "needs-triage", "type:feature"]),
        (FeedbackKind::Comment, vec!["feedback", "type:comment"]),
    ] {
        let mut actual = kind.labels().to_vec();
        actual.sort_unstable();
        assert_eq!(actual, expected, "{kind:?} labels");
    }
}

fn payload(kind: FeedbackKind, answers: serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "kind": kind,
        "title": "Contract fixture",
        "answers": answers,
        "environment": {
            "app_version": null, "os": null, "gpu": null, "policy_version": null, "logs": null
        },
        "screenshot_url": null
    })
}

fn assert_named_answers_match_form(kind: FeedbackKind, answers: serde_json::Value) {
    let parsed = serde_json::from_value::<Feedback>(payload(kind, answers.clone()));
    assert!(parsed.is_ok(), "{kind:?} must accept canonical answer IDs: {:?}", parsed.err());
    let issue = parsed.unwrap().to_issue().expect("complete canonical answers must produce a draft");
    let form = repo_file(&format!(".github/ISSUE_TEMPLATE/{}", kind.template()));
    let mut expected = String::new();
    let mut matched = 0;
    for field in form["body"].as_sequence().unwrap() {
        let Some(id) = field["id"].as_str() else {
            continue;
        };
        let Some(text) = answers.get(id).and_then(serde_json::Value::as_str) else {
            continue;
        };
        let heading = field["attributes"]["label"].as_str().expect("answer controls need a label");
        // Literal expected redaction for this synthetic fixture, not a call to the production redactor.
        let text = match text {
            "ann@example.com" => "<email>",
            "" => "_No response_",
            text => text,
        };
        expected.push_str(&format!("### {heading}\n\n{text}\n\n"));
        matched += 1;
    }
    assert_eq!(
        matched,
        answers.as_object().unwrap().len(),
        "{kind:?} IDs must all exist in the hosted form"
    );
    expected.push_str("<sub>Sent from CADE (Help › Send feedback).</sub>\n");
    assert_eq!(
        issue.body, expected,
        "{kind:?} must render redacted answers with canonical labels in form order"
    );
}

#[test]
fn to_issue_bug_named_keys_match_hosted_form_sections() {
    assert_named_answers_match_form(
        FeedbackKind::Bug,
        serde_json::json!({
            "what-happened": "ann@example.com",
            "steps": "Open the canvas",
            "expected": "The canvas opens",
            "actual": "The canvas is blank",
            "output": "System",
            "area": "Canvas",
            "severity": "Work is blocked"
        }),
    );
}

#[test]
fn to_issue_feature_named_keys_match_hosted_form_sections() {
    assert_named_answers_match_form(
        FeedbackKind::FeatureRequest,
        serde_json::json!({
            "problem": "ann@example.com",
            "proposal": "Add an export",
            "alternatives": "Export manually",
            "output": "System",
            "beneficiaries": "team",
            "contribute": ""
        }),
    );
}

#[test]
fn to_issue_comment_named_keys_match_hosted_form_sections() {
    assert_named_answers_match_form(
        FeedbackKind::Comment,
        serde_json::json!({
            "where": "Canvas",
            "comment": "ann@example.com",
            "output": "System",
            "reply": ""
        }),
    );
}

#[test]
fn from_value_legacy_heading_arrays_migrate_to_canonical_answer_keys() {
    let fixtures: Vec<serde_json::Value> = serde_json::from_str(include_str!("fixtures/legacy-feedback.json")).unwrap();
    for (kind, fields) in [
        (
            FeedbackKind::Bug,
            vec![
                ("What happened?", "what-happened"),
                ("Steps to reproduce", "steps"),
                ("What did you expect?", "expected"),
                ("Output type", "output"),
            ],
        ),
        (
            FeedbackKind::FeatureRequest,
            vec![
                ("What problem would this solve?", "problem"),
                ("What would you like CADE to do?", "proposal"),
                ("Which output type?", "output"),
                ("What do you do today instead?", "alternatives"),
            ],
        ),
        (
            FeedbackKind::Comment,
            vec![("Your comment", "comment"), ("Where in CADE?", "where")],
        ),
    ] {
        let legacy_answers: Vec<_> = fields
            .iter()
            .map(|(heading, id)| serde_json::json!({"heading": heading, "text": if *id == "output" { "App" } else { *id }}))
            .collect();
        let expected: serde_json::Map<String, serde_json::Value> = fields
            .iter()
            .map(|(_, id)| (id.to_string(), serde_json::json!(if *id == "output" { "App" } else { *id })))
            .collect();
        let legacy = fixtures.iter().find(|fixture| fixture["kind"] == serde_json::json!(kind)).unwrap();
        assert_eq!(
            legacy["answers"],
            serde_json::json!(legacy_answers),
            "{kind:?} legacy fixture retains every old heading"
        );
        let parsed = serde_json::from_value::<Feedback>(legacy.clone());
        assert!(parsed.is_ok(), "{kind:?} legacy arrays remain accepted");
        let feedback = parsed.unwrap();
        assert!(feedback.to_issue().is_ok(), "{kind:?} migrated required answers remain valid");
        let serialized = serde_json::to_value(feedback).unwrap();
        assert_eq!(
            serialized["answers"],
            serde_json::Value::Object(expected),
            "{kind:?} canonical migration"
        );
        assert_eq!(
            serialized["environment"], legacy["environment"],
            "{kind:?} environment survives migration"
        );
    }
}

fn required_answers() -> [(FeedbackKind, serde_json::Value); 3] {
    [
        (FeedbackKind::Bug, serde_json::json!({"what-happened": "Summary", "steps": "Open"})),
        (
            FeedbackKind::FeatureRequest,
            serde_json::json!({"problem": "Problem", "proposal": "Proposal"}),
        ),
        (FeedbackKind::Comment, serde_json::json!({"comment": "Comment"})),
    ]
}

fn assert_no_draft(json: serde_json::Value) {
    if let Ok(feedback) = serde_json::from_value::<Feedback>(json) {
        assert!(feedback.to_issue().is_err(), "invalid answers must not produce a publishable draft");
    }
}

#[test]
fn to_issue_named_required_answers_missing_or_whitespace_deny_drafts() {
    // Baseline controls: map rejection alone is not proof of keyed required-field validation.
    // The three positive named-key tests must also pass after migration.
    for (kind, answers) in required_answers() {
        for id in answers.as_object().unwrap().keys() {
            let mut missing = answers.clone();
            missing.as_object_mut().unwrap().remove(id);
            assert_no_draft(payload(kind, missing));
            let mut blank = answers.clone();
            blank[id] = serde_json::json!(" \n\t ");
            assert_no_draft(payload(kind, blank));
        }
    }
}

#[test]
fn to_issue_unknown_keys_cannot_replace_required_answers() {
    for (kind, answers) in required_answers() {
        let form = repo_file(&format!(".github/ISSUE_TEMPLATE/{}", kind.template()));
        for id in answers.as_object().unwrap().keys() {
            let field = form["body"]
                .as_sequence()
                .unwrap()
                .iter()
                .find(|field| field["id"].as_str() == Some(id.as_str()))
                .expect("required canonical ID must exist in the form");
            let heading = field["attributes"]["label"].as_str().unwrap();
            for unknown in ["unknown", heading] {
                let mut invalid = answers.clone();
                invalid.as_object_mut().unwrap().remove(id);
                invalid[unknown] = serde_json::json!("Not a canonical ID");
                assert_no_draft(payload(kind, invalid));
            }
        }
    }
}

#[test]
fn to_issue_duplicate_required_keys_or_legacy_headings_deny_drafts() {
    // Keep raw JSON: parsing through Value would discard duplicate map entries.
    for answers in [
        r#"{"comment":"","comment":"Filled"}"#,
        r#"{"comment":"Filled","comment":""}"#,
        r#"{"comment":"First","comment":"Second"}"#,
        r#"[{"heading":"Your comment","text":""},{"heading":"Your comment","text":"Filled"}]"#,
        r#"[{"heading":"Your comment","text":"Filled"},{"heading":"Your comment","text":""}]"#,
        r#"[{"heading":"Your comment","text":"First"},{"heading":"Your comment","text":"Second"}]"#,
    ] {
        let raw = format!(r#"{{"kind":"comment","title":"Contract fixture","answers":{answers},"environment":{{}}}}"#);
        if let Ok(feedback) = serde_json::from_str::<Feedback>(&raw) {
            assert!(feedback.to_issue().is_err(), "duplicate required answer accepted: {answers}");
        }
    }
}

#[test]
fn to_issue_unknown_answer_names_cannot_inject_output_headings() {
    let hostile = "ann@example.com\n\n### Injected ghp_abcdefghijklmnopqrstuvwxyz123456";
    for answers in [
        serde_json::json!({"comment": "Safe text", hostile: "Injected text"}),
        serde_json::json!([
            {"heading": "Your comment", "text": "Safe text"},
            {"heading": hostile, "text": "Injected text"}
        ]),
    ] {
        let Ok(feedback) = serde_json::from_value::<Feedback>(payload(FeedbackKind::Comment, answers)) else {
            continue;
        };
        let Ok(issue) = feedback.to_issue() else {
            continue;
        };
        assert_eq!(
            issue.body, "### Your comment\n\nSafe text\n\n<sub>Sent from CADE (Help › Send feedback).</sub>\n",
            "unknown answer names must be rejected or omitted, never become output headings"
        );
    }
}

#[test]
fn from_value_invalid_answer_shapes_are_rejected() {
    // Payload shape fixtures, not a substitute for JSON Schema validation of the hosted forms.
    for answers in [
        serde_json::json!(null),
        serde_json::json!("comment"),
        serde_json::json!({"comment": 7}),
        serde_json::json!({"comment": true}),
        serde_json::json!({"comment": ["text"]}),
        serde_json::json!({"comment": {"text": "text"}}),
        serde_json::json!([{"heading": "Your comment"}]),
        serde_json::json!([{"heading": "Your comment", "text": 7}]),
    ] {
        assert!(
            serde_json::from_value::<Feedback>(payload(FeedbackKind::Comment, answers)).is_err(),
            "answers must be keyed strings or complete legacy heading/text pairs"
        );
    }
}
