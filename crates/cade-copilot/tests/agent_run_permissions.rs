//! An agent run's tool calls pass through `PolicyPermissions` before a fake
//! executor runs them, so the test sees exactly what would have executed.

use async_trait::async_trait;
use cade_copilot::{PolicyPermissions, Prompt, PromptUser};
use cade_policy::Policy;
use github_copilot_sdk::handler::{PermissionHandler, PermissionResult};
use github_copilot_sdk::{PermissionRequestData, RequestId, SessionId};
use std::sync::{Arc, Mutex, RwLock};

/// Approves only the commands it's told to and records every prompt.
struct ScriptedUser {
    approve: Vec<&'static str>,
    asked: Arc<Mutex<Vec<String>>>,
}

#[async_trait]
impl PromptUser for ScriptedUser {
    async fn ask(&self, prompt: Prompt) -> bool {
        let Prompt::Ask { summary } = prompt;
        let ok = self.approve.contains(&summary.as_str());
        self.asked.lock().unwrap().push(summary);
        ok
    }
}

fn approved(r: &PermissionResult) -> bool {
    matches!(r, PermissionResult::Decision { decision, .. } if format!("{decision:?}").starts_with("ApproveOnce"))
}

async fn run(policy: &str, approve: Vec<&'static str>, calls: &[serde_json::Value]) -> (Vec<String>, Vec<String>) {
    let asked = Arc::new(Mutex::new(vec![]));
    let handler = PolicyPermissions {
        policy: Arc::new(RwLock::new(Policy::from_yaml(policy).unwrap())),
        prompt: ScriptedUser {
            approve,
            asked: asked.clone(),
        },
    };
    let mut executed = vec![];
    for (i, call) in calls.iter().enumerate() {
        let data: PermissionRequestData = serde_json::from_value(call.clone()).unwrap();
        let result = handler
            .handle(SessionId::new("run-1"), RequestId::new(format!("req-{i}")), data)
            .await;
        if approved(&result) {
            executed.push(call.to_string());
        }
    }
    let asked = asked.lock().unwrap().clone();
    (executed, asked)
}

#[tokio::test]
async fn policy_permissions_strict_policy_executes_only_allowed_and_user_approved_calls() {
    let policy = "version: 1\ntools: { shell: ask, preapproved: [cade-render], network: off, mcp: [azure-docs] }\n";
    let calls = [
        serde_json::json!({"kind": "shell", "command": "cade-render --check a11y"}),
        serde_json::json!({"kind": "shell", "command": "npm install chart-lib@4"}),
        serde_json::json!({"kind": "shell", "command": "rm -rf build"}),
        serde_json::json!({"kind": "url", "url": "https://example.test"}),
        serde_json::json!({"kind": "mcp", "serverName": "random-mcp"}),
        serde_json::json!({"kind": "read", "path": "project.cade"}),
    ];
    let (executed, asked) = run(policy, vec!["npm install chart-lib@4"], &calls).await;

    assert_eq!(
        executed,
        [&calls[0], &calls[1], &calls[5]].map(|c| c.to_string()).to_vec(),
        "network and the unlisted MCP server are blocked; rm is declined"
    );
    assert_eq!(asked, ["npm install chart-lib@4", "rm -rf build"]);
}

#[tokio::test]
async fn policy_permissions_shell_deny_executes_nothing_and_never_prompts() {
    let calls = [
        serde_json::json!({"kind": "shell", "command": "ls"}),
        serde_json::json!({"kind": "shell", "permissionRequest": {"fullCommandText": "git push --force"}}),
    ];
    let (executed, asked) = run("version: 1\ntools: { shell: deny }\n", vec!["ls", "git push --force"], &calls).await;
    assert_eq!(executed, Vec::<String>::new());
    assert_eq!(asked, Vec::<String>::new());
}
