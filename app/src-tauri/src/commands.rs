// Single declaration authority for native handlers, registration and standalone
// generation. Bracketed parameters are runtime-injected, never caller arguments.
commands! {
    /// The policy currently in force (cached from `<org>/.github/cade/policy.yml`).
    sync fn current_policy([state: tauri::State<'_, AppState>]) -> Result<serde_json::Value, CommandError> {
        let p = state.policy.lock().map_err(|e| e.to_string())?;
        serde_json::to_value(&*p).map_err(|e| e.to_string().into())
    }

    /// Ask the policy whether a shell command may run without a prompt.
    sync fn check_shell([state: tauri::State<'_, AppState>], command: String) -> Result<String, CommandError> {
        let p = state.policy.lock().map_err(|e| e.to_string())?;
        Ok(match p.decide(&Action::RunShell(&command)) {
            Decision::Allow => "allow".into(),
            Decision::Ask => "ask".into(),
            Decision::Deny { rule } => format!("deny:{rule}"),
        })
    }

    /// Build the issue the Feedback dialog previews. Sending happens in `submit_feedback`.
    sync fn preview_feedback([], feedback: Feedback) -> Result<IssueDraft, CommandError> {
        feedback.to_issue().map_err(|e| CommandError::Message(e.to_string()))
    }

    /// Create the issue on the CADE repo as the signed-in user.
    async fn submit_feedback([], feedback: Feedback) -> Result<String, CommandError> {
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
}
