//! IPC tests: each command is invoked through Tauri's mock runtime, the same
//! path the webview uses, with the state and handler list from `with_commands`.

use super::*;
use serde_json::{Value, json};
use tauri::ipc::{CallbackFn, InvokeBody};
use tauri::test::{INVOKE_KEY, MockRuntime, get_ipc_response, mock_builder, mock_context, noop_assets};
use tauri::webview::InvokeRequest;
use tauri::{Manager, WebviewWindow, WebviewWindowBuilder};

fn app() -> tauri::App<MockRuntime> {
    with_commands(mock_builder())
        .build(mock_context(noop_assets()))
        .expect("mock app builds")
}

fn window(app: &tauri::App<MockRuntime>) -> WebviewWindow<MockRuntime> {
    WebviewWindowBuilder::new(app, "main", Default::default())
        .build()
        .expect("mock window builds")
}

/// The app's own origin, which the ACL allows to call app commands. Tauri serves it as
/// `http://tauri.localhost` on Windows and Android and `tauri://localhost` elsewhere
/// (as in the `tauri::test` docs); any other origin is remote and rejected.
const LOCAL_ORIGIN: &str = if cfg!(any(windows, target_os = "android")) {
    "http://tauri.localhost"
} else {
    "tauri://localhost"
};

fn invoke(window: &WebviewWindow<MockRuntime>, cmd: &str, args: Value) -> Result<Value, Value> {
    invoke_from(window, LOCAL_ORIGIN, cmd, args)
}

fn invoke_from(window: &WebviewWindow<MockRuntime>, origin: &str, cmd: &str, args: Value) -> Result<Value, Value> {
    get_ipc_response(
        window,
        InvokeRequest {
            cmd: cmd.into(),
            callback: CallbackFn(0),
            error: CallbackFn(1),
            url: origin.parse().expect("valid url"),
            body: InvokeBody::Json(args),
            headers: Default::default(),
            invoke_key: INVOKE_KEY.to_string(),
        },
    )
    .map(|body| body.deserialize::<Value>().expect("json response"))
}

fn set_policy(app: &tauri::App<MockRuntime>, yaml: &str) {
    *app.state::<AppState>().policy.lock().expect("policy lock") = Policy::from_yaml(yaml).expect("valid policy");
}

fn bug(title: &str, first_answer: &str) -> Value {
    json!({
        "feedback": {
            "kind": "bug",
            "title": title,
            "answers": [
                { "heading": "What happened?", "text": first_answer },
                { "heading": "Steps to reproduce", "text": "Open a big file." }
            ],
            "environment": { "app_version": "0.1.0", "os": null, "gpu": null, "policy_version": null, "logs": null }
        }
    })
}

#[test]
fn current_policy_default_state_returns_default_policy_json() {
    let app = app();
    let w = window(&app);

    let policy = invoke(&w, "current_policy", json!({})).expect("command succeeds");

    assert_eq!(policy, serde_json::to_value(Policy::default()).expect("serializable"));
    assert_eq!(policy["tools"]["shell"], "ask");
    assert_eq!(policy["telemetry"], "off");
}

#[test]
fn current_policy_after_policy_change_returns_new_policy() {
    let app = app();
    let w = window(&app);
    set_policy(&app, "version: 3\ntools: { shell: deny }\n");

    let policy = invoke(&w, "current_policy", json!({})).expect("command succeeds");

    assert_eq!(policy["version"], 3);
    assert_eq!(policy["tools"]["shell"], "deny");
}

#[test]
fn check_shell_default_policy_returns_ask() {
    let app = app();
    let w = window(&app);

    assert_eq!(invoke(&w, "check_shell", json!({ "command": "git status" })), Ok(json!("ask")));
}

#[test]
fn check_shell_shell_denied_returns_deny_with_rule() {
    let app = app();
    let w = window(&app);
    set_policy(&app, "version: 1\ntools: { shell: deny }\n");

    assert_eq!(
        invoke(&w, "check_shell", json!({ "command": "rm -rf build" })),
        Ok(json!("deny:tools.shell = deny"))
    );
}

#[test]
fn check_shell_preapproved_command_returns_allow() {
    let app = app();
    let w = window(&app);
    set_policy(&app, "version: 1\ntools: { shell: deny, preapproved: [bicep] }\n");

    assert_eq!(
        invoke(&w, "check_shell", json!({ "command": "bicep build main.bicep" })),
        Ok(json!("allow"))
    );
}

#[test]
fn check_shell_missing_command_argument_returns_error() {
    let app = app();
    let w = window(&app);

    assert_eq!(
        invoke(&w, "check_shell", json!({})),
        Err(json!(
            "invalid args `command` for command `check_shell`: command check_shell missing required key command"
        ))
    );
}

#[test]
fn preview_feedback_valid_bug_returns_labeled_issue_draft() {
    let app = app();
    let w = window(&app);

    let draft = invoke(&w, "preview_feedback", bug("Canvas freezes", "It froze.")).expect("command succeeds");

    assert_eq!(draft["title"], "[Bug] Canvas freezes");
    assert_eq!(draft["labels"][0], "bug");
    let body = draft["body"].as_str().expect("body is a string");
    assert!(
        body.starts_with("### What happened?\n\nIt froze.\n\n### Steps to reproduce\n\nOpen a big file.\n\n"),
        "{body}"
    );
    assert!(body.contains("- **CADE version:** 0.1.0"), "{body}");
    assert!(!body.contains("Operating system"), "opted-out environment row was sent: {body}");
}

#[test]
fn preview_feedback_blank_title_returns_validation_message() {
    let app = app();
    let w = window(&app);

    assert_eq!(
        invoke(&w, "preview_feedback", bug("  ", "It froze.")),
        Err(json!({ "Message": "a title is required" }))
    );
}

#[test]
fn preview_feedback_unknown_kind_returns_error() {
    let app = app();
    let w = window(&app);
    let mut args = bug("Canvas freezes", "It froze.");
    args["feedback"]["kind"] = json!("rant");

    assert_eq!(
        invoke(&w, "preview_feedback", args),
        Err(json!(
            "invalid args `feedback` for command `preview_feedback`: unknown variant `rant`, expected one of `bug`, `feature-request`, `comment`"
        ))
    );
}

#[test]
fn submit_feedback_invalid_feedback_fails_before_reading_keychain() {
    let app = app();
    let w = window(&app);

    let result = invoke(&w, "submit_feedback", bug("Canvas freezes", " "));

    assert_eq!(result, Err(json!({ "Message": "\"What happened?\" is required" })));
}

#[test]
fn with_commands_unregistered_command_is_rejected() {
    let app = app();
    let w = window(&app);

    assert_eq!(
        invoke(&w, "delete_everything", json!({})),
        Err(json!("Command delete_everything not found"))
    );
}

#[test]
fn with_commands_remote_origin_is_not_allowed() {
    let app = app();
    let w = window(&app);

    assert_eq!(
        invoke_from(&w, "https://example.com", "check_shell", json!({ "command": "ls" })),
        Err(json!("check_shell not allowed. Plugin not found"))
    );
}
