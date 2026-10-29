//! Selected-host Git preflight and transient, explicitly configured SDK transport.

use std::sync::{Arc, Mutex};

use cade_policy::{Action, Decision, Policy};
use gix::protocol::transport::{
    Protocol,
    client::blocking_io::{Transport, http},
};
use octocrab::secrecy::{ExposeSecret, SecretString};
use url::Url;

use crate::{GitHubError, Host};

#[cfg(test)]
#[path = "../tests/transport_boundaries/mod.rs"]
mod boundaries;

impl Host {
    /// Construct an operation-private push remote and Host-owned security options.
    ///
    /// The repository must be a private handle. Its in-memory configuration is
    /// replaced, never persisted. Callers must use the returned options unchanged.
    pub fn git_push_remote<'repo, 'auth>(
        &'auth self,
        repository: &'repo git2::Repository,
        remote: &'auth url::Url,
        token: &'auth SecretString,
        policy: Option<&'auth Policy>,
        mut callbacks: git2::RemoteCallbacks<'auth>,
        interrupt: &'auth std::sync::atomic::AtomicBool,
    ) -> Result<(git2::Remote<'repo>, git2::PushOptions<'auth>), GitHubError> {
        self.validate_git_remote(remote, token, policy)?;
        if interrupt.load(std::sync::atomic::Ordering::Relaxed) {
            return Err(GitHubError::GitTransport("Git push interrupted before transport"));
        }
        if !cfg!(any(target_os = "macos", target_os = "linux")) {
            return Err(GitHubError::GitTransport("secure Git push is unsupported on this platform"));
        }
        let config = git2::Config::new().map_err(|_| GitHubError::GitTransport("cannot isolate Git push configuration"))?;
        repository
            .set_config(&config)
            .map_err(|_| GitHubError::GitTransport("cannot isolate Git push configuration"))?;
        let selected = repository
            .remote_anonymous(remote.as_str())
            .map_err(|_| GitHubError::GitTransport("cannot construct selected Git push remote"))?;
        if selected.url_bytes() != remote.as_str().as_bytes()
            || selected.pushurl_bytes().is_some_and(|url| url != remote.as_str().as_bytes())
        {
            return Err(GitHubError::GitTransport("Git push destination must equal the selected repository"));
        }
        callbacks.credentials(Self::push_credentials(self, remote, token, policy, interrupt));
        callbacks.certificate_check(|_, _| Ok(git2::CertificateCheckStatus::CertificatePassthrough));
        callbacks.sideband_progress(|_| !interrupt.load(std::sync::atomic::Ordering::Relaxed));
        let mut options = git2::PushOptions::new();
        options
            .remote_callbacks(callbacks)
            .proxy_options(git2::ProxyOptions::new())
            .follow_redirects(git2::RemoteRedirect::None)
            .custom_headers(&[])
            .remote_push_options(&[]);
        Ok((selected, options))
    }

    /// Validate the selected Git destination, token, and policy without I/O.
    ///
    /// Shared preflight lets clone reject requests before creating its destination
    /// and the connection factory reject requests before constructing transport.
    pub fn validate_git_remote(&self, remote: &url::Url, token: &SecretString, policy: Option<&Policy>) -> Result<(), GitHubError> {
        if !policy.is_some_and(|policy| policy.decide(&Action::AgentNetwork) == Decision::Allow) {
            return Err(GitHubError::GitTransport(
                "Git network access requires an explicitly allowing policy",
            ));
        }
        require_token(token)?;
        if let Host::GheCom(tenant) = self
            && (tenant.is_empty()
                || tenant.len() > 63
                || !tenant.as_bytes()[0].is_ascii_alphanumeric()
                || !tenant.as_bytes()[tenant.len() - 1].is_ascii_alphanumeric()
                || !tenant.bytes().all(|byte| byte.is_ascii_alphanumeric() || byte == b'-'))
        {
            return Err(GitHubError::GitTransport("select a valid GHE.com tenant label"));
        }
        let base = Url::parse(&self.web_base()).map_err(|_| GitHubError::GitTransport("select a valid HTTPS GitHub host"))?;
        if !https_origin(&base) || !https_origin(remote) || base.origin() != remote.origin() {
            return Err(GitHubError::GitTransport(
                "select a credential-free HTTPS repository on the chosen GitHub host; redirects and other origins are not allowed",
            ));
        }
        let base_path = base.path().trim_end_matches('/');
        if !base_path.split('/').skip(1).all(path_component) {
            return Err(GitHubError::GitTransport("select a GitHub host with a valid repository base path"));
        }
        let relative = remote
            .path()
            .strip_prefix(base_path)
            .and_then(|path| path.strip_prefix('/'))
            .ok_or(GitHubError::GitTransport(
                "select a repository beneath the chosen GitHub host's base path",
            ))?;
        let mut components = relative.split('/');
        let owner = components.next().unwrap_or_default();
        let repository = components.next().unwrap_or_default();
        if !path_component(owner) || !path_component(repository.strip_suffix(".git").unwrap_or(repository)) || components.next().is_some() {
            return Err(GitHubError::GitTransport(
                "select an owner/repository HTTPS URL without encoded separators, credentials, query, fragment or endpoint suffix",
            ));
        }
        Ok(())
    }

    fn push_credentials<'auth>(
        host: &'auth Host,
        selected: &'auth Url,
        token: &'auth SecretString,
        policy: Option<&'auth Policy>,
        interrupt: &'auth std::sync::atomic::AtomicBool,
    ) -> impl FnMut(&str, Option<&str>, git2::CredentialType) -> Result<git2::Cred, git2::Error> + 'auth {
        let mut offered = false;
        move |address, username, allowed| {
            if offered
                || interrupt.load(std::sync::atomic::Ordering::Relaxed)
                || host.validate_git_remote(selected, token, policy).is_err()
                || !push_challenge(selected, address, username, allowed)
            {
                return Err(git2::Error::from_str("Git push authentication denied"));
            }
            offered = true;
            git2::Cred::userpass_plaintext("x-access-token", token.expose_secret())
                .map_err(|_| git2::Error::from_str("Git push authentication denied"))
        }
    }

    /// Construct a credential-bound Git connection only for this selected host.
    ///
    /// A supplied allowing policy and nonempty, host-associated token are required.
    /// Credentials must remain transient and failures must have fixed diagnostics.
    pub fn git_connection<'remote, 'repo>(
        &self,
        remote: &'remote gix::Remote<'repo>,
        token: &SecretString,
        policy: Option<&Policy>,
    ) -> Result<gix::remote::Connection<'remote, 'static, 'repo, Box<dyn Transport + Send>>, GitHubError> {
        let address = remote
            .url(gix::remote::Direction::Fetch)
            .ok_or(GitHubError::GitTransport("supply an explicit Git repository URL"))?;
        let serialized = address.to_bstring();
        let address_text =
            std::str::from_utf8(&serialized).map_err(|_| GitHubError::GitTransport("supply a valid HTTPS Git repository URL"))?;
        let selected = Url::parse(address_text).map_err(|_| GitHubError::GitTransport("supply a valid HTTPS Git repository URL"))?;
        self.validate_git_remote(&selected, token, policy)?;
        connection_trace(remote)?;
        let transport = http::connect::<http::reqwest::Remote>(address.clone(), Protocol::V2, false);
        configure_connection(remote, Box::new(transport), token)
    }
}

fn push_challenge(selected: &Url, address: &str, username: Option<&str>, allowed: git2::CredentialType) -> bool {
    Url::parse(address).is_ok_and(|address| address == *selected)
        && username.is_none_or(|username| username == "x-access-token")
        && allowed.contains(git2::CredentialType::USER_PASS_PLAINTEXT)
}

// The existing SDK transport is injectable privately to observe connection-time
// configuration without exporting hooks or sending synthetic credentials.
fn configure_connection<'remote, 'repo>(
    remote: &'remote gix::Remote<'repo>,
    mut transport: Box<dyn Transport + Send>,
    token: &SecretString,
) -> Result<gix::remote::Connection<'remote, 'static, 'repo, Box<dyn Transport + Send>>, GitHubError> {
    require_token(token)?;
    connection_trace(remote)?;
    transport
        .set_identity(gix::sec::identity::Account {
            username: "x-access-token".into(),
            password: token.expose_secret().to_owned(),
            oauth_refresh_token: None,
        })
        .map_err(|_| GitHubError::GitTransport("cannot configure Git authentication; sign in to the selected host and retry"))?;
    let options = http::Options {
        follow_redirects: http::options::FollowRedirects::None,
        proxy: Some(String::new()),
        no_proxy: Some("*".into()),
        proxy_authenticate: None,
        extra_headers: Vec::new(),
        verbose: false,
        ssl_verify: true,
        ssl_ca_info: None,
        backend: Some(Arc::new(Mutex::new(http::reqwest::Options {
            configure_client: Some(Box::new(|builder| {
                Ok(builder
                    .tls_backend_native()
                    .tls_danger_accept_invalid_certs(false)
                    .tls_danger_accept_invalid_hostnames(false)
                    .no_proxy())
            })),
            configure_request: None,
        }))),
        ..Default::default()
    };
    Ok(remote
        .to_connection_with_transport(transport)
        .with_transport_options(Box::new(options))
        // The supplied identity is sent once. Authentication failure must not
        // invoke helpers, prompt, store/erase credentials, or retry another identity.
        .with_credentials(|_| Ok(None)))
}

fn require_token(token: &SecretString) -> Result<(), GitHubError> {
    if token.expose_secret().is_empty() {
        Err(GitHubError::NotSignedIn)
    } else {
        Ok(())
    }
}

fn https_origin(url: &Url) -> bool {
    url.scheme() == "https"
        && url.host_str().is_some()
        && url.username().is_empty()
        && url.password().is_none()
        && url.query().is_none()
        && url.fragment().is_none()
}

fn path_component(component: &str) -> bool {
    !component.is_empty()
        && component != "."
        && component != ".."
        && component
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
}

fn connection_trace(remote: &gix::Remote<'_>) -> Result<(), GitHubError> {
    // The SDK Connection copies this setting from its borrowed repository and
    // has no trace setter. Never construct a tracing connection or mutate callers.
    if remote
        .repo()
        .config_snapshot()
        .try_boolean("gitoxide.tracePacket")
        .map_err(|_| GitHubError::GitTransport("correct invalid Git packet tracing configuration before connecting"))?
        .unwrap_or(false)
    {
        return Err(GitHubError::GitTransport(
            "disable Git packet tracing before connecting with credentials",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_git_remote_valid_dotcom_repository_returns_ok() {
        let remote = url::Url::parse("https://github.com/synthetic/project.git").unwrap();
        let token = SecretString::from("synthetic-token-not-a-credential".to_owned());
        let policy = Policy::default();

        assert!(
            Host::Dotcom.validate_git_remote(&remote, &token, Some(&policy)).is_ok(),
            "a valid selected-host repository must pass zero-I/O preflight"
        );
    }

    #[test]
    fn push_credentials_selected_challenge_offers_once() {
        let selected = Url::parse("https://github.com/synthetic/project.git").unwrap();
        let token = SecretString::from("synthetic-token".to_owned());
        let policy = Policy::default();
        let interrupt = std::sync::atomic::AtomicBool::new(false);
        let mut credentials = Host::push_credentials(&Host::Dotcom, &selected, &token, Some(&policy), &interrupt);
        assert!(credentials(selected.as_str(), None, git2::CredentialType::USER_PASS_PLAINTEXT).is_ok());
    }

    #[test]
    fn git_push_remote_credentials_rejects_repeated_challenge() {
        let selected = Url::parse("https://github.com/synthetic/project.git").unwrap();
        let token = SecretString::from("synthetic-token".to_owned());
        let policy = Policy::default();
        let interrupt = std::sync::atomic::AtomicBool::new(false);
        let mut credentials = Host::push_credentials(&Host::Dotcom, &selected, &token, Some(&policy), &interrupt);
        assert!(credentials(selected.as_str(), None, git2::CredentialType::USER_PASS_PLAINTEXT).is_ok());
        assert!(
            credentials(selected.as_str(), None, git2::CredentialType::USER_PASS_PLAINTEXT).is_err(),
            "second token offer must be denied"
        );
    }

    fn rejected_challenge(address: &str, username: Option<&str>, kind: git2::CredentialType) {
        let selected = Url::parse("https://github.com/synthetic/project.git").unwrap();
        let token = SecretString::from("synthetic-token".to_owned());
        let policy = Policy::default();
        let interrupt = std::sync::atomic::AtomicBool::new(false);
        let mut credentials = Host::push_credentials(&Host::Dotcom, &selected, &token, Some(&policy), &interrupt);
        assert!(
            credentials(address, username, kind).is_err(),
            "unsupported challenge must not receive a token"
        );
    }

    #[test]
    fn git_push_remote_credentials_changed_url_returns_error() {
        rejected_challenge(
            "https://github.com/synthetic/other.git",
            None,
            git2::CredentialType::USER_PASS_PLAINTEXT,
        );
    }

    #[test]
    fn git_push_remote_credentials_other_username_returns_error() {
        rejected_challenge(
            "https://github.com/synthetic/project.git",
            Some("other"),
            git2::CredentialType::USER_PASS_PLAINTEXT,
        );
    }

    #[test]
    fn git_push_remote_credentials_other_type_returns_error() {
        rejected_challenge("https://github.com/synthetic/project.git", None, git2::CredentialType::SSH_KEY);
    }

    #[test]
    fn git_push_remote_credentials_missing_policy_returns_error() {
        let selected = Url::parse("https://github.com/synthetic/project.git").unwrap();
        let token = SecretString::from("synthetic-token".to_owned());
        let interrupt = std::sync::atomic::AtomicBool::new(false);
        let mut credentials = Host::push_credentials(&Host::Dotcom, &selected, &token, None, &interrupt);
        assert!(
            credentials(selected.as_str(), None, git2::CredentialType::USER_PASS_PLAINTEXT).is_err(),
            "callback must recheck supplied policy"
        );
    }

    #[test]
    fn git_push_remote_credentials_interrupted_challenge_returns_error() {
        let selected = Url::parse("https://github.com/synthetic/project.git").unwrap();
        let token = SecretString::from("synthetic-token".to_owned());
        let policy = Policy::default();
        let interrupt = std::sync::atomic::AtomicBool::new(false);
        let mut credentials = Host::push_credentials(&Host::Dotcom, &selected, &token, Some(&policy), &interrupt);
        interrupt.store(true, std::sync::atomic::Ordering::Relaxed);
        assert!(
            credentials(selected.as_str(), None, git2::CredentialType::USER_PASS_PLAINTEXT).is_err(),
            "callback must observe original interruption flag"
        );
    }
}
