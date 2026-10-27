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
}
