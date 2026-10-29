//! Selected-host boundary probes. Construction is lazy; no handshake or DNS.

use cade_github::{GitHubError, Host};
use cade_policy::{Policy, Switch};
use octocrab::secrecy::SecretString;
use url::Url;

fn connection(host: Host, address: &str, token: &str, policy: Option<&Policy>) -> Result<(), GitHubError> {
    let dir = tempfile::tempdir().unwrap();
    let repo = gix::ThreadSafeRepository::init_opts(
        dir.path(),
        gix::create::Kind::Bare,
        gix::create::Options::default(),
        gix::open::Options::isolated(),
    )
    .unwrap()
    .to_thread_local();
    let remote = repo.remote_at_without_url_rewrite(address).unwrap();
    let token = SecretString::from(token.to_owned());
    host.git_connection(&remote, &token, policy).map(|mut connection| {
        assert_eq!(
            connection.transport_mut().to_url().as_ref(),
            gix::bstr::BStr::new(address),
            "the factory must not rewrite the selected destination"
        );
    })
}

fn allowed(host: Host, address: &str) {
    assert!(
        connection(host, address, "synthetic-token-not-a-credential", Some(&Policy::default())).is_ok(),
        "a valid selected-host repository must construct a lazy connection"
    );
}

fn denied(host: Host, address: &str) {
    assert!(matches!(
        connection(host, address, "synthetic-token-not-a-credential", Some(&Policy::default())),
        Err(GitHubError::GitTransport(_))
    ));
}

#[test]
fn git_connection_dotcom_repository_constructs_connection() {
    allowed(Host::Dotcom, "https://github.com/synthetic/project.git");
}

#[test]
fn git_connection_ghe_tenant_repository_constructs_connection() {
    allowed(
        Host::GheCom("synthetic-tenant".into()),
        "https://synthetic-tenant.ghe.com/synthetic/project.git",
    );
}

#[test]
fn git_connection_server_port_and_base_path_constructs_connection() {
    allowed(
        Host::Server(Url::parse("https://forge.example.invalid:8443/github/").unwrap()),
        "https://forge.example.invalid:8443/github/synthetic/project.git",
    );
}

#[test]
fn git_connection_other_origin_returns_transport_error() {
    for address in [
        "https://other.example.invalid/synthetic/project.git",
        "https://github.com:8443/synthetic/project.git",
        "https://github.com.example.invalid/synthetic/project.git",
    ] {
        denied(Host::Dotcom, address);
    }
}

#[test]
fn git_connection_other_tenant_returns_transport_error() {
    denied(
        Host::GheCom("synthetic-tenant".into()),
        "https://other-tenant.ghe.com/synthetic/project.git",
    );
}

#[test]
fn git_connection_invalid_selected_host_returns_transport_error() {
    for tenant in ["", "-tenant", "tenant-", "a.b", "tenant/other", "tenant@other", "tenant_name"] {
        denied(
            Host::GheCom(tenant.into()),
            "https://synthetic-tenant.ghe.com/synthetic/project.git",
        );
    }
    for base in [
        "http://forge.example.invalid/",
        "https://user:synthetic-url-secret@forge.example.invalid/",
        "https://forge.example.invalid/?query",
        "https://forge.example.invalid/#fragment",
    ] {
        denied(
            Host::Server(Url::parse(base).unwrap()),
            "https://forge.example.invalid/synthetic/project.git",
        );
    }
}

#[test]
fn git_connection_outside_repository_path_returns_transport_error() {
    let host = Host::Server(Url::parse("https://forge.example.invalid/github/").unwrap());
    for address in [
        "https://forge.example.invalid/github",
        "https://forge.example.invalid/github/owner",
        "https://forge.example.invalid/github-other/owner/project.git",
        "https://forge.example.invalid/owner/project.git",
        "https://forge.example.invalid/github/owner/project.git/info/refs",
        "https://forge.example.invalid/github/owner%2fother/project.git",
        "https://forge.example.invalid/github/owner/%2e%2e/project.git",
    ] {
        denied(host.clone(), address);
    }
}

#[test]
fn git_connection_credential_or_suffix_url_returns_transport_error() {
    for address in [
        "https://user:synthetic-url-secret@github.com/synthetic/project.git",
        "https://github.com/synthetic/project.git?token=synthetic-query-secret",
        "https://github.com/synthetic/project.git#synthetic-fragment",
    ] {
        denied(Host::Dotcom, address);
    }
}

#[test]
fn git_connection_non_https_transport_returns_transport_error() {
    for address in [
        "http://github.com/synthetic/project.git",
        "ssh://git@github.com/synthetic/project.git",
        "git://github.com/synthetic/project.git",
        "file:///synthetic/project.git",
        "ext::synthetic-helper",
    ] {
        denied(Host::Dotcom, address);
    }
}

#[test]
fn git_connection_missing_policy_returns_transport_error() {
    assert!(matches!(
        connection(Host::Dotcom, "https://github.com/synthetic/project.git", "synthetic-token", None),
        Err(GitHubError::GitTransport(_))
    ));
}

#[test]
fn git_connection_network_denied_returns_transport_error() {
    let mut policy = Policy::default();
    policy.tools.network = Switch::Off;
    assert!(matches!(
        connection(
            Host::Dotcom,
            "https://github.com/synthetic/project.git",
            "synthetic-token",
            Some(&policy)
        ),
        Err(GitHubError::GitTransport(_))
    ));
}

#[test]
fn git_connection_empty_token_returns_not_signed_in() {
    assert!(matches!(
        connection(
            Host::Dotcom,
            "https://github.com/synthetic/project.git",
            "",
            Some(&Policy::default())
        ),
        Err(GitHubError::NotSignedIn)
    ));
}

#[test]
fn git_connection_rejected_input_keeps_display_and_debug_sanitized() {
    let error = connection(
        Host::Dotcom,
        "https://user:synthetic-url-secret@github.com/synthetic/project.git?synthetic-query-secret",
        "synthetic-token-not-a-credential",
        Some(&Policy::default()),
    )
    .unwrap_err();
    for rendered in [error.to_string(), format!("{error:?}")] {
        for forbidden in ["synthetic-url-secret", "synthetic-query-secret", "synthetic-token-not-a-credential"] {
            assert!(
                !rendered.contains(forbidden),
                "a transport error must not include input credentials"
            );
        }
    }
}
