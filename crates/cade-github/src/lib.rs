//! GitHub for CADE: sign-in, hosts, and the API calls CADE makes.
//!
//! One identity does everything. The user signs in once with the OAuth
//! device flow, and that token is used to:
//!
//! - start Copilot (the SDK also accepts the token),
//! - ship to pull requests, publish review pages, and sync review comments,
//! - file bugs, feature requests and comments on the CADE repo as themselves.
//!
//! Hosts: github.com, GHE.com (data residency) and GitHub Enterprise Server.
//!
//! Backlog: GH epic, FEED epic.

use octocrab::Octocrab;
use url::Url;

/// The repository in-app feedback goes to. Set at build time for forks.
pub const FEEDBACK_REPO: (&str, &str) = match option_env!("CADE_FEEDBACK_OWNER") {
    Some(o) => (o, "cade"),
    None => ("cade-dev", "cade"),
};

/// OAuth scopes CADE asks for. `public_repo` is requested only when the user
/// first sends feedback (scope upgrade prompt, FEED backlog).
pub const BASE_SCOPES: &[&str] = &["read:user", "read:org", "repo"];
pub const FEEDBACK_SCOPES: &[&str] = &["public_repo"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Host {
    Dotcom,
    /// `<subdomain>.ghe.com`
    GheCom(String),
    /// GitHub Enterprise Server root, e.g. `https://github.contoso.com`.
    Server(Url),
}

impl Host {
    pub fn api_base(&self) -> String {
        match self {
            Host::Dotcom => "https://api.github.com".into(),
            Host::GheCom(sub) => format!("https://api.{sub}.ghe.com"),
            Host::Server(u) => format!("{}/api/v3", u.as_str().trim_end_matches('/')),
        }
    }

    pub fn web_base(&self) -> String {
        match self {
            Host::Dotcom => "https://github.com".into(),
            Host::GheCom(sub) => format!("https://{sub}.ghe.com"),
            Host::Server(u) => u.as_str().trim_end_matches('/').to_string(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GitHubError {
    #[error("GitHub API: {0}")]
    Api(#[from] octocrab::Error),
    #[error("not signed in")]
    NotSignedIn,
    #[error("keychain: {0}")]
    Keychain(String),
}

pub mod auth {
    //! Device-flow sign-in and token storage in the OS keychain.
    use super::*;
    use octocrab::secrecy::SecretString;

    const SERVICE: &str = "dev.cade.app";
    const ACCOUNT: &str = "github-token";

    pub fn store_token(token: &str) -> Result<(), GitHubError> {
        keyring::Entry::new(SERVICE, ACCOUNT)
            .and_then(|e| e.set_password(token))
            .map_err(|e| GitHubError::Keychain(e.to_string()))
    }

    pub fn stored_token() -> Result<String, GitHubError> {
        match keyring::Entry::new(SERVICE, ACCOUNT).and_then(|e| e.get_password()) {
            Ok(t) => Ok(t),
            Err(keyring::Error::NoEntry) => Err(GitHubError::NotSignedIn),
            Err(e) => Err(GitHubError::Keychain(e.to_string())),
        }
    }

    pub fn sign_out() -> Result<(), GitHubError> {
        keyring::Entry::new(SERVICE, ACCOUNT)
            .and_then(|e| e.delete_credential())
            .map_err(|e| GitHubError::Keychain(e.to_string()))
    }

    /// What the first-run screen shows while the user approves in the browser.
    pub struct Pending {
        pub user_code: String,
        pub verification_uri: String,
        codes: octocrab::auth::DeviceCodes,
        crab: Octocrab,
        client_id: SecretString,
    }

    /// Start the device flow. `client_id` is CADE's OAuth app.
    pub async fn start(host: &Host, client_id: &str, scopes: &[&str]) -> Result<Pending, GitHubError> {
        let crab = Octocrab::builder()
            .base_uri(host.web_base())?
            .add_header("accept".parse().unwrap(), "application/json".into())
            .build()?;
        let client_id = SecretString::from(client_id.to_string());
        let codes = crab.authenticate_as_device(&client_id, scopes).await?;
        Ok(Pending {
            user_code: codes.user_code.clone(),
            verification_uri: codes.verification_uri.clone(),
            codes,
            crab,
            client_id,
        })
    }

    impl Pending {
        /// Wait for approval, then store the token.
        pub async fn finish(self) -> Result<(), GitHubError> {
            use octocrab::secrecy::ExposeSecret;
            let oauth = self.codes.poll_until_available(&self.crab, &self.client_id).await?;
            store_token(oauth.access_token.expose_secret())
        }
    }
}

/// A pull request CADE opens when shipping.
pub struct PullRequest<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub head: &'a str,
    pub base: &'a str,
    pub title: &'a str,
    pub body: &'a str,
    pub draft: bool,
}

pub struct Client {
    crab: Octocrab,
    pub host: Host,
}

impl Client {
    pub fn new(host: Host, token: &str) -> Result<Self, GitHubError> {
        let crab = Octocrab::builder()
            .base_uri(host.api_base())?
            .personal_token(token.to_string())
            .build()?;
        Ok(Client { crab, host })
    }

    /// Login of the signed-in user, e.g. `jdavis`.
    pub async fn login(&self) -> Result<String, GitHubError> {
        Ok(self.crab.current().user().await?.login)
    }

    /// Create an issue and return its web URL.
    pub async fn create_issue(&self, owner: &str, repo: &str, title: &str, body: &str, labels: &[String]) -> Result<String, GitHubError> {
        let issue = self
            .crab
            .issues(owner, repo)
            .create(title)
            .body(body)
            .labels(labels.to_vec())
            .send()
            .await?;
        Ok(issue.html_url.to_string())
    }

    /// Open issues whose title matches, for the duplicate check before submitting.
    pub async fn similar_issues(&self, owner: &str, repo: &str, query: &str) -> Result<Vec<(u64, String)>, GitHubError> {
        let q = format!("repo:{owner}/{repo} is:issue is:open in:title {query}");
        let page = self.crab.search().issues_and_pull_requests(&q).per_page(5).send().await?;
        Ok(page.items.into_iter().map(|i| (i.number, i.title)).collect())
    }

    /// Open a pull request for a shipped project.
    pub async fn open_pull_request(&self, pr: &PullRequest<'_>) -> Result<String, GitHubError> {
        let created = self
            .crab
            .pulls(pr.owner, pr.repo)
            .create(pr.title, pr.head, pr.base)
            .body(pr.body)
            .draft(pr.draft)
            .send()
            .await?;
        Ok(created.html_url.map(|u| u.to_string()).unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hosts_map_to_api_bases() {
        assert_eq!(Host::Dotcom.api_base(), "https://api.github.com");
        assert_eq!(Host::GheCom("contoso".into()).api_base(), "https://api.contoso.ghe.com");
        assert_eq!(
            Host::Server(Url::parse("https://github.contoso.com/").unwrap()).api_base(),
            "https://github.contoso.com/api/v3"
        );
    }

    #[test]
    fn web_base_each_host_maps_to_its_sign_in_root() {
        assert_eq!(Host::Dotcom.web_base(), "https://github.com");
        assert_eq!(Host::GheCom("contoso".into()).web_base(), "https://contoso.ghe.com");
        assert_eq!(
            Host::Server(Url::parse("https://github.contoso.com/").unwrap()).web_base(),
            "https://github.contoso.com"
        );
    }

    #[test]
    fn api_base_server_under_a_path_keeps_the_path_without_double_slash() {
        let host = Host::Server(Url::parse("https://tools.contoso.com/github/").unwrap());
        assert_eq!(host.api_base(), "https://tools.contoso.com/github/api/v3");
        assert_eq!(host.web_base(), "https://tools.contoso.com/github");
    }

    #[test]
    fn scopes_feedback_scope_is_requested_separately_from_base_scopes() {
        assert_eq!(BASE_SCOPES, ["read:user", "read:org", "repo"]);
        assert_eq!(FEEDBACK_SCOPES, ["public_repo"]);
        assert!(!BASE_SCOPES.contains(&"public_repo"));
    }

    #[test]
    fn feedback_goes_to_the_cade_repo() {
        assert_eq!(FEEDBACK_REPO.1, "cade");
    }
}
