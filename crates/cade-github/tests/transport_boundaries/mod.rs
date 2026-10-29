//! Connection-time configuration probes using the SDK's existing Transport trait.
//! No network, TLS replacement, keychain access, or exported test hook.

use super::configure_connection;
use crate::GitHubError;
use gix::protocol::transport::{
    Service,
    client::{
        Error, MessageKind, TransportWithoutIO, WriteMode,
        blocking_io::{RequestWriter, SetServiceResponse, Transport, http},
    },
};
use octocrab::secrecy::SecretString;
use std::{
    any::Any,
    borrow::Cow,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
};

struct Probe {
    options: Arc<Mutex<Vec<http::Options>>>,
    identity_matches: Arc<AtomicBool>,
    handshakes: Arc<AtomicUsize>,
}

impl TransportWithoutIO for Probe {
    fn to_url(&self) -> Cow<'_, gix::bstr::BStr> {
        Cow::Borrowed(gix::bstr::BStr::new("https://forge.example.invalid/synthetic/project.git"))
    }

    fn set_identity(&mut self, identity: gix::sec::identity::Account) -> Result<(), Error> {
        // Record only equality, never retain or format the SDK account or token.
        self.identity_matches.store(
            identity.username == "x-access-token" && identity.password == "synthetic-token" && identity.oauth_refresh_token.is_none(),
            Ordering::SeqCst,
        );
        Ok(())
    }

    fn connection_persists_across_multiple_requests(&self) -> bool {
        false
    }

    fn configure(&mut self, config: &dyn Any) -> gix::Result {
        let options = config
            .downcast_ref::<http::Options>()
            .expect("explicit SDK HTTP options at handshake");
        self.options.lock().unwrap().push(options.clone());
        Ok(())
    }
}

impl Transport for Probe {
    fn handshake<'a>(
        &mut self,
        _service: Service,
        _extra_parameters: &'a [(&'a str, Option<&'a str>)],
    ) -> Result<SetServiceResponse<'_>, Error> {
        self.handshakes.fetch_add(1, Ordering::SeqCst);
        Err(Error::Io(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "synthetic authentication rejection",
        )))
    }

    fn request(&mut self, _write_mode: WriteMode, _on_into_read: MessageKind, _trace: bool) -> Result<RequestWriter<'_>, Error> {
        panic!("a rejected handshake must not reach a Git request")
    }
}

fn configured(check: impl FnOnce(&http::Options, bool, usize)) {
    let dir = tempfile::tempdir().unwrap();
    let marker = dir.path().join("helper-was-invoked");
    let repo = gix::ThreadSafeRepository::init_opts(
        dir.path().join("repo"),
        gix::create::Kind::Bare,
        gix::create::Options::default(),
        gix::open::Options::isolated().config_overrides([
            "http.followRedirects=true".to_owned(),
            "http.sslVerify=false".to_owned(),
            "http.proxy=http://proxy.example.invalid:9".to_owned(),
            "http.extraHeader=X-Synthetic-Config: forbidden".to_owned(),
            "gitoxide.http.verbose=true".to_owned(),
            format!("credential.helper=!mkdir \"{}\"", marker.display()),
        ]),
    )
    .unwrap()
    .to_thread_local();
    let remote = repo
        .remote_at_without_url_rewrite("https://forge.example.invalid/synthetic/project.git")
        .unwrap()
        .with_refspecs(["+refs/heads/*:refs/remotes/origin/*"], gix::remote::Direction::Fetch)
        .unwrap();
    let options = Arc::new(Mutex::new(Vec::new()));
    let identity_matches = Arc::new(AtomicBool::new(false));
    let handshakes = Arc::new(AtomicUsize::new(0));
    let probe = Probe {
        options: options.clone(),
        identity_matches: identity_matches.clone(),
        handshakes: handshakes.clone(),
    };
    let connection = configure_connection(&remote, Box::new(probe), &SecretString::from("synthetic-token".to_owned()));
    assert!(connection.is_ok(), "approved private composition must configure the SDK connection");
    let result = connection
        .unwrap()
        .prepare_fetch(gix::progress::Discard, gix::remote::ref_map::Options::default());
    assert!(result.is_err(), "the fixture always rejects authentication");
    drop(result);
    let options = options.lock().unwrap();
    assert_eq!(
        options.len(),
        1,
        "options must reach the SDK at connection handshake, not just factory construction"
    );
    assert!(!marker.exists(), "configured credential helpers must never execute");
    check(
        &options[0],
        identity_matches.load(Ordering::SeqCst),
        handshakes.load(Ordering::SeqCst),
    );
}

#[test]
fn configure_connection_redirect_override_disables_redirects() {
    configured(|options, _, _| {
        assert_eq!(options.follow_redirects, http::options::FollowRedirects::None);
    });
}

#[test]
fn configure_connection_proxy_override_disables_proxy_authentication() {
    configured(|options, _, _| {
        assert!(
            options.proxy.as_deref() == Some("") || options.no_proxy.as_deref() == Some("*"),
            "disable proxies explicitly rather than inheriting environment or configuration"
        );
        assert!(options.proxy_authenticate.is_none());
    });
}

#[test]
fn configure_connection_header_and_trace_overrides_discards_configured_values() {
    configured(|options, _, _| {
        assert!(options.extra_headers.is_empty());
        assert!(!options.verbose);
    });
}

#[test]
fn configure_connection_tls_override_keeps_verification_enabled() {
    configured(|options, _, _| {
        assert!(options.ssl_verify);
        assert!(
            options.ssl_ca_info.is_none(),
            "repository configuration must not replace native trust"
        );
        let backend = options.backend.as_ref().expect("explicit native backend configuration");
        let backend = backend.lock().unwrap();
        let reqwest = backend
            .downcast_ref::<http::reqwest::Options>()
            .expect("pinned SDK reqwest options");
        assert!(
            reqwest.configure_client.is_some(),
            "native TLS/no-proxy callback must be explicitly supplied"
        );
        // The existence of this callback is not proof of its behavior or OS trust.
    });
}

#[test]
fn configure_connection_authentication_rejection_never_falls_back_to_helpers() {
    configured(|_, identity_matches, handshakes| {
        assert!(identity_matches, "use the supplied transient x-access-token identity");
        assert_eq!(handshakes, 1, "no automatic retry with helper credentials");
    });
}

#[test]
fn configure_connection_empty_token_returns_not_signed_in() {
    let dir = tempfile::tempdir().unwrap();
    let repo = gix::init_bare(dir.path()).unwrap();
    let remote = repo
        .remote_at_without_url_rewrite("https://forge.example.invalid/synthetic/project.git")
        .unwrap();
    let probe = Probe {
        options: Arc::new(Mutex::new(Vec::new())),
        identity_matches: Arc::new(AtomicBool::new(false)),
        handshakes: Arc::new(AtomicUsize::new(0)),
    };
    assert!(matches!(
        configure_connection(&remote, Box::new(probe), &SecretString::from(String::new())),
        Err(GitHubError::NotSignedIn)
    ));
}
