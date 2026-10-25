//! `Client` and device-flow calls against a local mock GitHub Enterprise
//! Server. Nothing here reaches GitHub or the OS keychain.

use cade_github::{Client, GitHubError, Host, PullRequest, auth};
use serde_json::{Value, json};
use url::Url;
use wiremock::matchers::{body_json, header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

const TOKEN: &str = "gho_test_token";

fn fixture(name: &str) -> Value {
    let path = format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
    serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap()
}

fn host(server: &MockServer) -> Host {
    Host::Server(Url::parse(&server.uri()).unwrap())
}

fn client(server: &MockServer) -> Client {
    Client::new(host(server), TOKEN).unwrap()
}

fn error_body(message: &str) -> Value {
    json!({ "message": message, "documentation_url": "https://docs.github.com/rest" })
}

fn status_of(err: GitHubError) -> u16 {
    match err {
        GitHubError::Api(octocrab::Error::GitHub { source, .. }) => source.status_code.as_u16(),
        other => panic!("expected a GitHub API error, got {other:?}"),
    }
}

#[tokio::test]
async fn login_signed_in_user_returns_login_with_bearer_token() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v3/user"))
        .and(header("authorization", format!("Bearer {TOKEN}").as_str()))
        .respond_with(ResponseTemplate::new(200).set_body_json(fixture("user.json")))
        .expect(1)
        .mount(&server)
        .await;

    assert_eq!(client(&server).login().await.unwrap(), "jdavis");
}

#[tokio::test]
async fn login_bad_token_returns_401_api_error() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v3/user"))
        .respond_with(ResponseTemplate::new(401).set_body_json(error_body("Bad credentials")))
        .mount(&server)
        .await;

    assert_eq!(status_of(client(&server).login().await.unwrap_err()), 401);
}

#[tokio::test]
async fn create_issue_posts_title_body_and_labels_and_returns_html_url() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/v3/repos/cade-dev/cade/issues"))
        .and(header("authorization", format!("Bearer {TOKEN}").as_str()))
        .and(body_json(json!({
            "title": "[Bug] Canvas freezes",
            "body": "### What happened?\n\nIt froze.",
            "labels": ["bug", "from-app"]
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(fixture("issue.json")))
        .expect(1)
        .mount(&server)
        .await;

    let url = client(&server)
        .create_issue(
            "cade-dev",
            "cade",
            "[Bug] Canvas freezes",
            "### What happened?\n\nIt froze.",
            &["bug".into(), "from-app".into()],
        )
        .await
        .unwrap();

    assert_eq!(url, "https://github.example.test/cade-dev/cade/issues/42");
}

#[tokio::test]
async fn create_issue_repo_not_found_returns_404_api_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/v3/repos/cade-dev/missing/issues"))
        .respond_with(ResponseTemplate::new(404).set_body_json(error_body("Not Found")))
        .mount(&server)
        .await;

    let err = client(&server)
        .create_issue("cade-dev", "missing", "t", "b", &[])
        .await
        .unwrap_err();
    assert_eq!(status_of(err), 404);
}

#[tokio::test]
async fn create_issue_validation_failure_returns_422_api_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/v3/repos/cade-dev/cade/issues"))
        .respond_with(ResponseTemplate::new(422).set_body_json(error_body("Validation Failed")))
        .mount(&server)
        .await;

    let err = client(&server)
        .create_issue("cade-dev", "cade", "t", "b", &["no-such-label".into()])
        .await
        .unwrap_err();
    assert_eq!(status_of(err), 422);
}

#[tokio::test]
async fn create_issue_server_error_returns_5xx_api_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/v3/repos/cade-dev/cade/issues"))
        .respond_with(ResponseTemplate::new(503).set_body_json(error_body("Service Unavailable")))
        .mount(&server)
        .await;

    let err = client(&server).create_issue("cade-dev", "cade", "t", "b", &[]).await.unwrap_err();
    assert_eq!(status_of(err), 503);
}

#[tokio::test]
async fn similar_issues_searches_open_issues_by_title_and_maps_number_and_title() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v3/search/issues"))
        .and(query_param("q", "repo:cade-dev/cade is:issue is:open in:title canvas freezes"))
        .and(query_param("per_page", "5"))
        .respond_with(ResponseTemplate::new(200).set_body_json(fixture("search_issues.json")))
        .expect(1)
        .mount(&server)
        .await;

    let found = client(&server).similar_issues("cade-dev", "cade", "canvas freezes").await.unwrap();

    assert_eq!(
        found,
        vec![(42, "[Bug] Canvas freezes".to_string()), (7, "Canvas freezes on zoom".to_string())]
    );
}

#[tokio::test]
async fn similar_issues_no_matches_returns_empty() {
    let server = MockServer::start().await;
    let mut empty = fixture("search_issues.json");
    empty["total_count"] = json!(0);
    empty["items"] = json!([]);
    Mock::given(method("GET"))
        .and(path("/api/v3/search/issues"))
        .respond_with(ResponseTemplate::new(200).set_body_json(empty))
        .mount(&server)
        .await;

    assert!(client(&server).similar_issues("cade-dev", "cade", "zzz").await.unwrap().is_empty());
}

#[tokio::test]
async fn open_pull_request_posts_head_base_body_and_draft_and_returns_html_url() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/v3/repos/acme/designs/pulls"))
        .and(header("authorization", format!("Bearer {TOKEN}").as_str()))
        .and(body_json(json!({
            "title": "Ship onboarding screens",
            "head": "cade/onboarding",
            "base": "main",
            "body": "Shipped from CADE.",
            "draft": true
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(fixture("pull_request.json")))
        .expect(1)
        .mount(&server)
        .await;

    let url = client(&server)
        .open_pull_request(&PullRequest {
            owner: "acme",
            repo: "designs",
            head: "cade/onboarding",
            base: "main",
            title: "Ship onboarding screens",
            body: "Shipped from CADE.",
            draft: true,
        })
        .await
        .unwrap();

    assert_eq!(url, "https://github.example.test/acme/designs/pull/12");
}

#[tokio::test]
async fn open_pull_request_response_without_html_url_returns_missing_field_error() {
    let server = MockServer::start().await;
    let mut created = fixture("pull_request.json");
    created.as_object_mut().unwrap().remove("html_url");
    Mock::given(method("POST"))
        .and(path("/api/v3/repos/acme/designs/pulls"))
        .respond_with(ResponseTemplate::new(201).set_body_json(created))
        .expect(1)
        .mount(&server)
        .await;

    let result = client(&server)
        .open_pull_request(&PullRequest {
            owner: "acme",
            repo: "designs",
            head: "cade/onboarding",
            base: "main",
            title: "t",
            body: "b",
            draft: false,
        })
        .await;

    assert!(matches!(result, Err(GitHubError::MissingField("html_url"))), "got {result:?}");
}

#[tokio::test]
async fn open_pull_request_existing_pr_for_branch_returns_422_api_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/v3/repos/acme/designs/pulls"))
        .respond_with(ResponseTemplate::new(422).set_body_json(error_body("A pull request already exists")))
        .mount(&server)
        .await;

    let err = client(&server)
        .open_pull_request(&PullRequest {
            owner: "acme",
            repo: "designs",
            head: "cade/onboarding",
            base: "main",
            title: "t",
            body: "b",
            draft: false,
        })
        .await
        .unwrap_err();
    assert_eq!(status_of(err), 422);
}

#[tokio::test]
async fn start_device_flow_posts_client_id_and_joined_scopes_and_returns_user_code() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/login/device/code"))
        .and(header("accept", "application/json"))
        .and(body_json(json!({ "client_id": "Iv1.test", "scope": "read:user,read:org,repo" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "device_code": "3584d83530557fdd1f46af8289938c8ef79f9dc5",
            "user_code": "WDJB-MJHT",
            "verification_uri": "https://github.example.test/login/device",
            "expires_in": 900,
            "interval": 5
        })))
        .expect(1)
        .mount(&server)
        .await;

    let pending = auth::start(&host(&server), "Iv1.test", cade_github::BASE_SCOPES).await.unwrap();

    assert_eq!(pending.user_code, "WDJB-MJHT");
    assert_eq!(pending.verification_uri, "https://github.example.test/login/device");
}

#[tokio::test]
async fn start_device_flow_unknown_client_returns_api_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/login/device/code"))
        .respond_with(ResponseTemplate::new(404).set_body_json(error_body("Not Found")))
        .mount(&server)
        .await;

    let err = auth::start(&host(&server), "Iv1.unknown", cade_github::BASE_SCOPES)
        .await
        .err()
        .expect("start should fail");
    assert_eq!(status_of(err), 404);
}
