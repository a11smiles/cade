//! Private runner transport checks, not a second implementation of feedback rules.
//! Build `generate_feedback` before running this integration target on its own.

use cade_feedback::Feedback;
use serde_json::{Value, json};
use std::io::Write;
use std::process::{Command, Stdio};

fn preview(input: &str) -> Value {
    let executable = std::env::current_exe()
        .expect("integration test executable")
        .parent()
        .expect("deps directory")
        .parent()
        .expect("profile directory")
        .join("examples")
        .join(format!("generate_feedback{}", std::env::consts::EXE_SUFFIX));
    let mut child = Command::new(executable)
        .arg("preview")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("build generate_feedback before running this target");
    child.stdin.take().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "preview transport must serialize modeled errors");
    assert!(
        output.stderr.is_empty(),
        "preview transport must not print input or errors to stderr"
    );
    serde_json::from_slice(&output.stdout).expect("stdout is one JSON envelope")
}

#[test]
fn preview_valid_payloads_return_actual_to_issue_drafts() {
    for (kind, answers) in [
        (
            "bug",
            json!({"what-happened": "ann@example.com", "steps": "Open /Users/ann/project", "output": "System, App"}),
        ),
        (
            "feature-request",
            json!({"problem": "A problem", "proposal": "A proposal", "beneficiaries": "team"}),
        ),
        ("comment", json!({"comment": "A comment", "reply": "Yes"})),
    ] {
        let input = json!({
            "kind": kind,
            "title": "Contact ann@example.com",
            "answers": answers,
            "environment": {"app_version": "0.1.0"},
            "screenshot_url": "https://example.com/image.png"
        });
        let feedback: Feedback = serde_json::from_value(input.clone()).unwrap();
        let expected = json!({"ok": feedback.to_issue().unwrap()});
        assert_eq!(preview(&input.to_string()), expected, "{kind}");
    }
}

#[test]
fn preview_invalid_feedback_returns_actual_to_issue_errors() {
    for input in [
        json!({"kind": "comment", "title": " ", "answers": {"comment": "Comment"}, "environment": {}}),
        json!({"kind": "comment", "title": "Title", "answers": {}, "environment": {}}),
        json!({"kind": "comment", "title": "Title", "answers": {"comment": "Comment", "output": "Unknown"}, "environment": {}}),
        json!({
            "kind": "comment", "title": "Title", "answers": {"comment": "Comment"},
            "environment": {}, "screenshot_url": "http://example.com/image.png"
        }),
    ] {
        let feedback: Feedback = serde_json::from_value(input.clone()).unwrap();
        let expected = json!({"error": {"Message": feedback.to_issue().unwrap_err().to_string()}});
        assert_eq!(preview(&input.to_string()), expected);
    }
}

#[test]
fn preview_malformed_or_wrong_shape_input_returns_content_free_error() {
    for input in [
        r#"{"kind":"private-input-marker""#,
        r#"{"kind":"private-input-marker","title":"Title","answers":{},"environment":{}}"#,
        r#"{"kind":"comment","title":"Title","answers":{"comment":42},"environment":{}}"#,
        "null",
        "",
    ] {
        assert_eq!(preview(input), json!({"error": {"Message": "invalid feedback JSON"}}));
    }
}
