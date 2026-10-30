//! Private runner transport checks, not a second implementation of feedback rules.
//! Build `generate_feedback` before running this integration target on its own.

use cade_feedback::Feedback;
use serde_json::{Value, json};
use std::io::Write;
use std::process::{Command, Stdio};

fn generator() -> Command {
    let executable = std::env::current_exe()
        .expect("integration test executable")
        .parent()
        .expect("deps directory")
        .parent()
        .expect("profile directory")
        .join("examples")
        .join(format!("generate_feedback{}", std::env::consts::EXE_SUFFIX));
    Command::new(executable)
}

fn preview(input: &str) -> Value {
    let mut child = generator()
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
fn bindings_explicit_output_matches_stdout_bytes() {
    let fixture = tempfile::tempdir().expect("owned output directory");
    let path = fixture.path().join("bindings.ts");
    let stdout = generator().arg("bindings").output().expect("run stdout generation");
    assert!(stdout.status.success());
    let written = generator().arg("bindings").arg(&path).output().expect("run explicit generation");
    assert!(written.status.success());
    assert_eq!(
        std::fs::read(&path).unwrap_or_default(),
        stdout.stdout,
        "an explicit output path must receive the complete standalone artifact"
    );
    assert!(written.stdout.is_empty(), "file generation must not also print the artifact");
}

#[test]
fn bindings_unwritable_output_returns_failure() {
    let fixture = tempfile::tempdir().expect("owned output directory");
    let output = generator()
        .arg("bindings")
        .arg(fixture.path())
        .output()
        .expect("run generation with a directory as output");
    assert!(!output.status.success(), "an unwritable output must not report generation success");
    assert!(
        output.stdout.is_empty(),
        "failed file generation must not masquerade as stdout success"
    );
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
