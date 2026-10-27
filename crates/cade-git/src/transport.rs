//! Explicit clone/fetch routing and shared SDK connection composition.

use std::{
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
};

use cade_github::{GitHubError, Host};
use cade_policy::Policy;
use gix::protocol::transport::client::blocking_io::Transport;
use octocrab::secrecy::SecretString;
use url::Url;

use crate::{GitError, ProjectRepo};

mod clone;
mod fetch;

#[cfg(test)]
#[path = "../tests/transport_boundaries/mod.rs"]
mod boundaries;

impl ProjectRepo {
    /// Clone the explicitly selected HTTPS repository into an absent destination.
    ///
    /// Requires an associated token and an allowing policy. Failures must retain
    /// partial state for explicit recovery rather than delete destination content.
    /// Initial checkout and index publication precede local branch/HEAD publication.
    /// Progress reports client-owned stages and counts, never backend diagnostics.
    pub fn clone<P>(
        destination: &Path,
        remote: &Url,
        host: &Host,
        token: &SecretString,
        policy: Option<&Policy>,
        progress: P,
        interrupt: &AtomicBool,
    ) -> Result<Self, GitError>
    where
        P: gix::NestedProgress,
        P::SubProgress: 'static,
    {
        // Zero-I/O preflight MUST precede even the atomic destination claim.
        host.validate_git_remote(remote, token, policy).map_err(github_error)?;
        Self::clone_with_connection(destination, remote, progress, interrupt, |remote| {
            host.git_connection(remote, token, policy).map_err(github_error)
        })
    }

    /// Fetch remote objects and tracking refs without changing local selections.
    ///
    /// Does not check out, merge, reset, or automatically retry failed work.
    /// Progress reports client-owned stages and counts, never backend diagnostics.
    pub fn fetch<P>(
        &self,
        remote: &Url,
        host: &Host,
        token: &SecretString,
        policy: Option<&Policy>,
        progress: P,
        interrupt: &AtomicBool,
    ) -> Result<gix::remote::fetch::Outcome, GitError>
    where
        P: gix::NestedProgress,
        P::SubProgress: 'static,
    {
        host.validate_git_remote(remote, token, policy).map_err(github_error)?;
        self.fetch_with_connection(remote, progress, interrupt, |remote| {
            host.git_connection(remote, token, policy).map_err(github_error)
        })
    }

    // Only transport construction differs in the filesystem/protocol probes.
    // Destination, receive, refs and checkout are the public path's composition.
    #[cfg(test)]
    fn clone_with_transport<P>(
        destination: &Path,
        remote: &Url,
        transport: Box<dyn Transport + Send>,
        progress: P,
        interrupt: &AtomicBool,
    ) -> Result<Self, GitError>
    where
        P: gix::NestedProgress,
        P::SubProgress: 'static,
    {
        Self::clone_with_connection(destination, remote, progress, interrupt, |remote| {
            Ok(remote
                .to_connection_with_transport(transport)
                .with_transport_options(Box::new(()))
                .with_credentials(|_| Ok(None)))
        })
    }

    // No exported testing API or alternate production protocol is introduced.
    #[cfg(test)]
    fn fetch_with_transport<P>(
        &self,
        remote: &Url,
        transport: Box<dyn Transport + Send>,
        progress: P,
        interrupt: &AtomicBool,
    ) -> Result<gix::remote::fetch::Outcome, GitError>
    where
        P: gix::NestedProgress,
        P::SubProgress: 'static,
    {
        self.fetch_with_connection(remote, progress, interrupt, |remote| {
            Ok(remote
                .to_connection_with_transport(transport)
                .with_transport_options(Box::new(()))
                .with_credentials(|_| Ok(None)))
        })
    }
}

fn failure(message: &'static str) -> GitError {
    GitError::Git(message.into())
}

fn github_error(error: GitHubError) -> GitError {
    // Do not forward Display/Debug for any SDK/API/keychain error, including
    // future variants. Only this crate's fixed transport messages may cross.
    match error {
        GitHubError::NotSignedIn => failure("not signed in; supply a token associated with the selected GitHub host"),
        GitHubError::GitTransport(message) => failure(message),
        _ => failure("GitHub connection failed; verify sign-in, selected host and policy before explicitly retrying"),
    }
}

fn interrupted(interrupt: &AtomicBool) -> Result<(), GitError> {
    if interrupt.load(Ordering::Relaxed) {
        Err(failure(
            "Git operation interrupted; downloaded objects or published refs may be retained; inspect state and explicitly retry",
        ))
    } else {
        Ok(())
    }
}

fn open_options() -> gix::open::Options {
    // Transient overrides, never serialized into the origin's configuration.
    // No environment/global/includes, executable helpers, packet trace or
    // fabricated reflog author. The API does not accept a reflog actor.
    gix::open::Options::isolated()
        .strict_config(true)
        .config_overrides(["gitoxide.tracePacket=false", "core.logAllRefUpdates=false"])
}

fn repository(repo: &gix::Repository) -> Result<(), GitError> {
    if repo.namespace().is_some() {
        return Err(failure(
            "Git fetch requires the default reference namespace; remove the namespace configuration and retry",
        ));
    }
    repo.head()
        .map_err(|_| failure("cannot read Git HEAD; correct repository metadata and explicitly retry"))?;
    crate::reject_executable_filters(repo)?;
    let config = repo.config_snapshot();
    for section in config.plumbing().sections_by_name("filter").into_iter().flatten() {
        if section.body().value_implicit("smudge").is_some() {
            return Err(failure(
                "Git transfer does not support executable smudge filters; remove the filter configuration and explicitly retry",
            ));
        }
    }
    Ok(())
}

fn tracking_remote<'repo>(repo: &'repo gix::Repository, remote: &Url) -> Result<gix::Remote<'repo>, GitError> {
    repo.remote_at_without_url_rewrite(remote.as_str())
        .and_then(|remote| remote.with_refspecs(["+refs/heads/*:refs/remotes/origin/*"], gix::remote::Direction::Fetch))
        .map(|remote| remote.with_fetch_tags(gix::remote::fetch::Tags::None))
        .map_err(|_| failure("cannot prepare explicit Git remote-tracking refspecs; correct the repository URL and retry"))
}
