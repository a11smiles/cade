//! CADE desktop shell.
//!
//! The UI (app/ui, React + TypeScript) calls these commands over Tauri IPC.
//! The GPU canvas is a native wgpu surface owned by `cade-render`, placed
//! under the transparent webview (FOUND/CANVAS backlog); until that lands
//! the UI draws a placeholder canvas.

use cade_feedback::{Feedback, IssueDraft};
use cade_policy::{Action, Decision, Policy};
use serde::Serialize;
use std::sync::Mutex;

#[derive(Default)]
pub struct AppState {
    policy: Mutex<Policy>,
}

#[derive(Debug, Serialize, thiserror::Error)]
pub enum CommandError {
    #[error("{0}")]
    Message(String),
}

impl From<String> for CommandError {
    fn from(s: String) -> Self {
        CommandError::Message(s)
    }
}

/// The policy currently in force (cached from `<org>/.github/cade/policy.yml`).
#[tauri::command]
fn current_policy(state: tauri::State<'_, AppState>) -> Result<serde_json::Value, CommandError> {
    let p = state.policy.lock().map_err(|e| e.to_string())?;
    serde_json::to_value(&*p).map_err(|e| e.to_string().into())
}

/// Ask the policy whether a shell command may run without a prompt.
#[tauri::command]
fn check_shell(state: tauri::State<'_, AppState>, command: String) -> Result<String, CommandError> {
    let p = state.policy.lock().map_err(|e| e.to_string())?;
    Ok(match p.decide(&Action::RunShell(&command)) {
        Decision::Allow => "allow".into(),
        Decision::Ask => "ask".into(),
        Decision::Deny { rule } => format!("deny:{rule}"),
    })
}

/// Build the issue the Feedback dialog previews. Sending happens in `submit_feedback`.
#[tauri::command]
fn preview_feedback(feedback: Feedback) -> Result<IssueDraft, CommandError> {
    feedback.to_issue().map_err(|e| CommandError::Message(e.to_string()))
}

/// Create the issue on the CADE repo as the signed-in user.
#[tauri::command]
async fn submit_feedback(feedback: Feedback) -> Result<String, CommandError> {
    let draft = feedback.to_issue().map_err(|e| CommandError::Message(e.to_string()))?;
    let token = cade_github::auth::stored_token().map_err(|e| CommandError::Message(e.to_string()))?;
    let client = cade_github::Client::new(cade_github::Host::Dotcom, &token).map_err(|e| CommandError::Message(e.to_string()))?;
    let url = client
        .create_issue(
            cade_github::FEEDBACK_REPO.0,
            cade_github::FEEDBACK_REPO.1,
            &draft.title,
            &draft.body,
            &draft.labels,
        )
        .await
        .map_err(|e| CommandError::Message(e.to_string()))?;
    Ok(url)
}

pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            current_policy,
            check_shell,
            preview_feedback,
            submit_feedback
        ])
        .run(tauri::generate_context!())
        .expect("error while running CADE");
}
