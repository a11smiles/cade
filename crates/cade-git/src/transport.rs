//! Explicit clone/fetch/push routing and shared SDK connection composition.

use std::{
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
};

use cade_github::{GitHubError, Host};
use cade_policy::Policy;
use gix::progress::{Count, Progress};
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
    /// Non-force push of one explicitly selected committed branch.
    ///
    /// This bounded implementation is incomplete and must not be published.
    #[expect(
        clippy::too_many_arguments,
        reason = "Keep explicit ref selection and security inputs without a new public wrapper"
    )]
    pub fn push<P>(
        &self,
        reference: &str,
        expected: gix::ObjectId,
        destination: &str,
        remote: &Url,
        host: &Host,
        token: &SecretString,
        policy: Option<&Policy>,
        mut progress: P,
        interrupt: &AtomicBool,
    ) -> Result<gix::ObjectId, GitError>
    where
        P: gix::NestedProgress,
    {
        // Shared zero-I/O preflight must precede all push preparation.
        host.validate_git_remote(remote, token, policy).map_err(github_error)?;
        crate::commit::validate_branch_reference(reference)
            .map_err(|_| failure("Git push requires a valid full source branch under refs/heads/"))?;
        crate::commit::validate_branch_reference(destination)
            .map_err(|_| failure("Git push requires a valid full destination branch under refs/heads/"))?;
        interrupted(interrupt)?;
        progress.info("Git push preparation started".to_owned());
        self.push_with_source(
            reference,
            expected,
            || {},
            |repo| {
                let expected_sdk = git2::Oid::from_bytes(expected.as_slice())
                    .map_err(|_| failure("Git push requires a supported immutable commit identifier"))?;
                push_objects(repo, expected, interrupt)?;
                let source = expected.to_string();
                let wire = format!("{source}:{destination}");
                let spec = gix::refspec::parse(wire.as_str().into(), gix::refspec::parse::Operation::Push)
                    .map_err(|_| failure("cannot prepare immutable Git push refspec"))?
                    .to_owned();
                let wire = String::from_utf8(spec.to_ref().to_bstring().to_vec())
                    .map_err(|_| failure("cannot serialize immutable Git push refspec"))?;
                let sdk = git2::Repository::open(repo.git_dir()).map_err(|_| failure("cannot open private Git push handle"))?;
                let mut negotiations = 0;
                let mut statuses = 0;
                let mut invalid = false;
                let mut pack = progress.add_child("Git push pack objects");
                let mut objects = progress.add_child("Git push upload objects");
                let mut bytes = progress.add_child("Git push upload bytes");
                let mut refs = progress.add_child("Git push verified refs");
                pack.init(None, gix::progress::count("objects"));
                objects.init(None, gix::progress::count("objects"));
                bytes.init(None, gix::progress::bytes());
                refs.init(Some(1), gix::progress::count("refs"));
                let mut callbacks = git2::RemoteCallbacks::new();
                // These SDK callbacks return (), not a cancellation result.
                // Forward only numeric counts, never provider-controlled text.
                callbacks.pack_progress(|_, current, total| {
                    pack.init(Some(total), gix::progress::count("objects"));
                    pack.set(current);
                });
                callbacks.push_transfer_progress(|current, total, transferred| {
                    objects.init(Some(total), gix::progress::count("objects"));
                    objects.set(current);
                    bytes.set(transferred);
                });
                callbacks.push_negotiation(|updates| {
                    #[cfg(test)]
                    if let Some(observer) = boundaries::push::BEFORE_NEGOTIATION.take() {
                        observer();
                    }
                    negotiations += 1;
                    #[cfg(test)]
                    for update in updates {
                        boundaries::push::NEGOTIATIONS.with_borrow_mut(|seen| {
                            seen.push((update.src(), update.dst(), update.src_refname_bytes().to_vec()));
                        });
                    }
                    // Project SDK-owned updates into borrowed fields; the same guard
                    // also accepts safe synthetic callback inputs in library tests.
                    let validate = |count: usize, src: &[u8], dst: &[u8], new: Option<git2::Oid>| {
                        if interrupt.load(Ordering::Relaxed)
                            || count != 1
                            || src != source.as_bytes()
                            || dst != destination.as_bytes()
                            || new != Some(expected_sdk)
                            || new.is_none_or(|oid| oid.is_zero())
                        {
                            return Err(git2::Error::from_str("Git push negotiation rejected"));
                        }
                        Ok(())
                    };
                    #[cfg(test)]
                    if let Some(count) = boundaries::push::NEGOTIATION_COUNT.get() {
                        return boundaries::push::NEGOTIATION_SOURCE.with_borrow(|src| {
                            boundaries::push::NEGOTIATION_DESTINATION
                                .with_borrow(|dst| validate(count, src, dst, boundaries::push::NEGOTIATION_OID.get()))
                        });
                    }
                    validate(
                        updates.len(),
                        updates.first().map_or(&[], |update| update.src_refname_bytes()),
                        updates.first().map_or(&[], |update| update.dst_refname_bytes()),
                        updates.first().map(|update| update.dst()),
                    )
                });
                callbacks.push_update_reference(|name, status| {
                    #[cfg(test)]
                    boundaries::push::STATUSES.with_borrow_mut(|seen| seen.push((name.to_owned(), status.map(str::to_owned))));
                    let mut report = |name: &str, status: Option<&str>| {
                        statuses += 1;
                        invalid |= name != destination || status.is_some() || interrupt.load(Ordering::Relaxed);
                    };
                    #[cfg(test)]
                    if boundaries::push::STATUS_OVERRIDE.get() {
                        for (name, status) in boundaries::push::STATUS_INPUT.with_borrow(Clone::clone) {
                            report(&name, status.as_deref());
                        }
                        return Ok(());
                    }
                    report(name, status);
                    #[cfg(test)]
                    if boundaries::push::STATUS_CALLBACK_ERROR.get() {
                        return Err(git2::Error::from_str("synthetic post-publication callback failure"));
                    }
                    Ok(())
                });
                let (selected, mut options) = host
                    .git_push_remote(&sdk, remote, token, policy, callbacks, interrupt)
                    .map_err(github_error)?;
                // Only library-test builds can substitute an owned local SDK transport.
                // Security options returned by Host remain untouched.
                #[cfg(test)]
                let selected = match boundaries::push::DESTINATION.with_borrow(Clone::clone) {
                    Some(path) => sdk
                        .remote_anonymous(
                            &boundaries::push::RECEIVE_URL
                                .with_borrow(Clone::clone)
                                .unwrap_or_else(|| path.join(".git").to_str().expect("owned fixture path").to_owned()),
                        )
                        .unwrap(),
                    None => selected,
                };
                let mut selected = selected;
                let observe = || observe_push(repo.git_dir(), destination, remote, host, token, policy, interrupt);
                progress.info("Git push remote inspection started".to_owned());
                if observe()? == Some(expected) {
                    refs.set(1);
                    progress.done("Git push intended remote tip verified".to_owned());
                    return Ok(expected);
                }
                interrupted(interrupt)?;
                progress.info("Git push upload started".to_owned());
                #[cfg(test)]
                boundaries::push::WIRES.with_borrow_mut(|seen| seen.push(wire.clone()));
                let uploaded = selected.push(&[wire], Some(&mut options));
                #[cfg(test)]
                boundaries::push::UPLOAD_RESULTS.with_borrow_mut(|seen| seen.push(uploaded.is_ok()));
                #[cfg(test)]
                if let Some(observer) = boundaries::push::AFTER_UPLOAD.take() {
                    observer();
                }
                drop(options);
                if interrupt.load(Ordering::Relaxed) {
                    return Err(failure(
                        "Git push outcome is uncertain; remote refs may have changed; verify before retrying",
                    ));
                }
                progress.info("Git push remote verification started".to_owned());
                let observed = observe();
                if negotiations != 1 || statuses != 1 || invalid {
                    return Err(failure(
                        "Git push rejected or incomplete protocol reports; inspect remote state before retrying",
                    ));
                }
                let observed =
                    observed.map_err(|_| failure("Git push outcome is uncertain; remote refs may have changed; verify before retrying"))?;
                if uploaded.is_err() && observed == Some(expected) {
                    return Err(failure(
                        "Git push failed; intended remote tip is now observed; reconcile before another action",
                    ));
                }
                if uploaded.is_err() || observed != Some(expected) {
                    return Err(failure(
                        "Git push did not verify the intended remote tip; inspect remote state before an explicit non-force retry",
                    ));
                }
                refs.set(1);
                progress.done("Git push intended remote tip verified".to_owned());
                Ok(expected)
            },
        )
    }

    fn push_with_source(
        &self,
        reference: &str,
        expected: gix::ObjectId,
        after_lock: impl FnOnce(),
        operation: impl FnOnce(&gix::Repository) -> Result<gix::ObjectId, GitError>,
    ) -> Result<gix::ObjectId, GitError> {
        let repo = gix::open_opts(self.repo.git_dir(), open_options().open_path_as_is(true))
            .map_err(|_| failure("cannot reopen Git push source; correct repository metadata and explicitly retry"))?;
        repository(&repo)?;
        let validate = |repo: &gix::Repository| {
            let observed = repo
                .try_find_reference(reference)
                .map_err(|_| failure("cannot inspect Git push source branch; correct repository metadata and explicitly retry"))?;
            if observed.and_then(|reference| reference.try_id().map(|id| id.detach())) != Some(expected) {
                return Err(failure(
                    "Git push requires an existing direct source branch matching the selected commit",
                ));
            }
            Ok(())
        };
        validate(&repo)?;
        repo.find_object(expected)
            .map_err(|_| failure("cannot read selected Git push commit; restore the object and explicitly retry"))?
            .try_into_commit()
            .map_err(|_| failure("Git push requires a commit object; explicitly select a committed branch"))?
            .decode()
            .map_err(|_| failure("cannot decode selected Git push commit; restore the object and explicitly retry"))?;
        let sdk = git2::Repository::open(repo.git_dir())
            .map_err(|_| failure("cannot reopen Git push source for locking; correct repository metadata and explicitly retry"))?;
        let mut guard = sdk
            .transaction()
            .map_err(|_| failure("cannot prepare Git push source lock transaction"))?;
        guard
            .lock_ref(reference)
            .map_err(|_| failure("cannot lock Git push source branch; release the owning operation before explicitly retrying"))?;
        after_lock();
        let repo = gix::open_opts(self.repo.git_dir(), open_options().open_path_as_is(true))
            .map_err(|_| failure("cannot reopen locked Git push source; correct repository metadata and explicitly retry"))?;
        validate(&repo)?;
        #[cfg(test)]
        if let Some(observer) = boundaries::push::AFTER_VALIDATION.take() {
            observer();
        }
        let result = operation(&repo);
        // A no-edit transaction releases only its own source marker.
        // Never commit it, set a target, or lock HEAD/index/worktree metadata.
        drop(guard);
        result
    }

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

fn observe_push(
    path: &Path,
    destination: &str,
    url: &Url,
    host: &Host,
    token: &SecretString,
    policy: Option<&Policy>,
    interrupt: &AtomicBool,
) -> Result<Option<gix::ObjectId>, GitError> {
    interrupted(interrupt)?;
    let repo = gix::open_opts(path, open_options().open_path_as_is(true)).map_err(|_| failure("cannot reopen Git push observer"))?;
    let remote = repo
        .remote_at_without_url_rewrite(url.as_str())
        .map_err(|_| failure("cannot prepare Git push observation"))?
        .with_fetch_tags(gix::remote::fetch::Tags::None);
    let options = gix::remote::ref_map::Options {
        prefix_from_spec_as_filter_on_remote: false,
        ..Default::default()
    };
    #[cfg(test)]
    if let Some(path) = boundaries::push::DESTINATION.with_borrow(Clone::clone) {
        host.validate_git_remote(url, token, policy).map_err(github_error)?;
        boundaries::push::QUERIES.set(boundaries::push::QUERIES.get() + 1);
        if let Some(observer) = boundaries::push::BEFORE_QUERY.take() {
            observer();
        }
        let query = |transport| {
            remote
                .to_connection_with_transport(transport)
                .ref_map(gix::progress::Discard, options)
        };
        let result = if boundaries::push::QUERY_DISCONNECTED_AT.get() == boundaries::push::QUERIES.get() {
            query(boundaries::disconnected())
        } else {
            let (result, server) = boundaries::with_upload_pack(&path, None, query);
            assert!(server.status.success(), "owned upload-pack query failed");
            result
        };
        interrupted(interrupt)?;
        let (map, _) = result.map_err(|_| failure("fresh Git push observation failed"))?;
        if boundaries::push::QUERY_OVERRIDE_AT.get() == boundaries::push::QUERIES.get() {
            return boundaries::push::QUERY_REFS.with_borrow(|refs| push_destination(refs, destination));
        }
        return push_destination(&map.remote_refs, destination);
    }
    let (map, _) = host
        .git_connection(&remote, token, policy)
        .map_err(github_error)?
        .ref_map(gix::progress::Discard, options)
        .map_err(|_| failure("fresh Git push observation failed"))?;
    interrupted(interrupt)?;
    push_destination(&map.remote_refs, destination)
}

fn push_destination(refs: &[gix::protocol::handshake::Ref], destination: &str) -> Result<Option<gix::ObjectId>, GitError> {
    let mut matching = refs.iter().filter(|reference| reference.unpack().0 == destination.as_bytes());
    let first = matching.next();
    if matching.next().is_some() {
        return Err(failure("ambiguous Git push destination advertisement"));
    }
    match first {
        None => Ok(None),
        Some(gix::protocol::handshake::Ref::Direct { object, .. }) => Ok(Some(*object)),
        _ => Err(failure("Git push destination must be a direct remote branch")),
    }
}

fn push_objects(repo: &gix::Repository, expected: gix::ObjectId, interrupt: &AtomicBool) -> Result<(), GitError> {
    let read = |id: gix::ObjectId, kind: gix::objs::Kind| {
        interrupted(interrupt)?;
        let object = repo
            .find_object(id)
            .map_err(|_| failure("cannot read required Git push object; restore objects and explicitly retry"))?;
        if object.kind != kind {
            return Err(failure(
                "required Git push object has an invalid type; restore objects and explicitly retry",
            ));
        }
        let data = gix::objs::Data::new(&object.data, object.kind, id.kind());
        data.verify_checksum(&id)
            .map_err(|_| failure("required Git push object checksum is invalid; restore objects and explicitly retry"))?;
        data.decode()
            .map_err(|_| failure("cannot decode required Git push object; restore objects and explicitly retry"))?;
        Ok(object)
    };
    // SDK traversal respects shallow boundaries, but never substitutes HEAD,
    // the index, replacement content, submodule objects or LFS payloads.
    let commits = repo
        .rev_walk([expected])
        .use_commit_graph(false)
        .all()
        .map_err(|_| failure("cannot traverse Git push ancestry; restore objects and explicitly retry"))?;
    for commit in commits {
        interrupted(interrupt)?;
        let commit = commit.map_err(|_| failure("cannot traverse required Git push commit; restore objects and explicitly retry"))?;
        let commit = read(commit.id, gix::objs::Kind::Commit)?
            .try_into_commit()
            .map_err(|_| failure("Git push ancestry requires commit objects"))?;
        let tree = read(
            commit
                .tree_id()
                .map_err(|_| failure("cannot read required Git push tree identifier"))?
                .detach(),
            gix::objs::Kind::Tree,
        )?
        .try_into_tree()
        .map_err(|_| failure("Git push requires tree objects"))?;
        let entries = tree
            .traverse()
            .breadthfirst
            .files()
            .map_err(|_| failure("cannot traverse required Git push tree; restore objects and explicitly retry"))?;
        for entry in entries {
            interrupted(interrupt)?;
            if entry.mode.is_commit() {
                continue;
            }
            let kind = if entry.mode.is_tree() {
                gix::objs::Kind::Tree
            } else {
                gix::objs::Kind::Blob
            };
            read(entry.oid, kind)?;
        }
    }
    Ok(())
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
