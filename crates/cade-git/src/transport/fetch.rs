//! Download and tracking-only publication without selecting or rewriting local state.

use gix::{
    Progress,
    bstr::ByteSlice,
    progress::Count,
    remote::fetch::{Outcome, Status, refs::update::Mode},
};

use super::*;

impl ProjectRepo {
    pub(super) fn fetch_with_connection<P, F>(
        &self,
        remote: &Url,
        progress: P,
        interrupt: &AtomicBool,
        connect: F,
    ) -> Result<Outcome, GitError>
    where
        P: gix::NestedProgress,
        P::SubProgress: 'static,
        F: for<'remote, 'repo> FnOnce(
            &'remote gix::Remote<'repo>,
        )
            -> Result<gix::remote::Connection<'remote, 'static, 'repo, Box<dyn Transport + Send>>, GitError>,
    {
        interrupted(interrupt)?;
        let mut repo = gix::open_opts(self.repo.git_dir(), open_options().open_path_as_is(true))
            .map_err(|_| failure("cannot reopen Git repository metadata/configuration; correct it and explicitly retry fetch"))?;
        repository(&repo)?;
        repo.refs.write_reflog = gix::refs::store::WriteReflog::Disable;
        let remote = tracking_remote(&repo, remote)?;
        receive(&repo, &remote, progress, interrupt, connect)
    }
}

pub(super) fn receive<P, F>(
    repo: &gix::Repository,
    remote: &gix::Remote<'_>,
    mut progress: P,
    interrupt: &AtomicBool,
    connect: F,
) -> Result<Outcome, GitError>
where
    P: gix::NestedProgress,
    P::SubProgress: 'static,
    F: for<'remote, 'repo> FnOnce(
        &'remote gix::Remote<'repo>,
    ) -> Result<gix::remote::Connection<'remote, 'static, 'repo, Box<dyn Transport + Send>>, GitError>,
{
    interrupted(interrupt)?;
    let connection = connect(remote)?;
    let mut advertisement = progress.add_child("Git advertisement");
    advertisement.init(None, gix::progress::count("refs"));
    advertisement.info("Git advertisement started".to_string());
    // SDK progress may contain server-controlled task names and sideband text.
    // Only fixed client stages and actual numeric outcomes reach the caller.
    let preparation = connection.prepare_fetch(
        gix::progress::Discard,
        gix::remote::ref_map::Options {
            // Include HEAD in the advertisement, without a writable HEAD mapping.
            prefix_from_spec_as_filter_on_remote: false,
            ..Default::default()
        },
    );
    // Handshake/ls-refs has no interrupt argument in this pinned SDK. Check the
    // caller's flag immediately after it returns, including on transport failure.
    interrupted(interrupt)?;
    let preparation = preparation.map_err(|_| {
        failure(
            "Git advertisement failed; verify connectivity, selected-host token, native TLS trust and repository URL; \
             redirects are disabled; correct the cause and explicitly retry fetch",
        )
    })?;
    let advertised_refs = preparation.ref_map().remote_refs.len();
    advertisement.set(advertised_refs);
    advertisement.info(format!("Git advertisement received: {advertised_refs} refs"));
    drop(advertisement);
    validate_mappings(repo, preparation.ref_map())?;
    interrupted(interrupt)?;
    let mut objects = progress.add_child("Git pack objects");
    objects.init(None, gix::progress::count("objects"));
    objects.info("Git pack receive and tracking publication started".to_string());
    let received = preparation.receive(gix::progress::Discard, interrupt);
    interrupted(interrupt)?;
    let outcome = received.map_err(|_| {
        failure(
            "Git fetch could not download or publish tracking refs; partial objects/refs may be retained; \
             check connectivity, authentication, TLS trust, remote objects and competing ref locks, then explicitly retry fetch",
        )
    })?;
    let object_count = match &outcome.status {
        Status::Change { write_pack_bundle, .. } => write_pack_bundle.index.num_objects as usize,
        Status::NoPackReceived { .. } => 0,
    };
    objects.set(object_count);
    objects.info(format!("Git pack receive returned: {object_count} indexed objects"));
    let mut refs = progress.add_child("Git tracking verification");
    refs.init(Some(outcome.ref_map.mappings.len()), gix::progress::count("refs"));
    refs.info("Git tracking verification started".to_string());
    validate_outcome(repo, &outcome)?;
    interrupted(interrupt)?;
    refs.set(outcome.ref_map.mappings.len());
    refs.info(format!("Git tracking refs verified: {}", outcome.ref_map.mappings.len()));
    progress.info("Git objects and tracking refs received".to_string());
    Ok(outcome)
}

fn validate_mappings(repo: &gix::Repository, map: &gix::remote::fetch::RefMap) -> Result<(), GitError> {
    if map.mappings.is_empty() {
        return Err(failure(
            "remote has no fetchable branches: it is empty/unborn or its HEAD/default branch is unavailable; existing refs are retained",
        ));
    }
    if !map.fixes.is_empty() {
        return Err(failure(
            "Git fetch rejected invalid remote reference mappings; correct the remote before explicitly retrying",
        ));
    }
    for mapping in &map.mappings {
        let source = mapping
            .remote
            .as_name()
            .ok_or_else(|| failure("Git fetch requires named remote branches, not object refspecs"))?;
        let local = mapping
            .local
            .as_ref()
            .ok_or_else(|| failure("Git fetch requires explicit remote-tracking destinations"))?;
        if !source.starts_with(b"refs/heads/")
            || !local.starts_with(b"refs/remotes/origin/")
            || source.strip_prefix(b"refs/heads/") != local.strip_prefix(b"refs/remotes/origin/")
            || gix::refs::FullName::try_from(local.clone()).is_err()
            || gix::validate::reference::branch_name(source).is_err()
        {
            return Err(failure(
                "Git fetch rejected a mapping outside the explicit origin remote-tracking namespace",
            ));
        }
        if mapping.remote.as_target().is_some() {
            return Err(failure(
                "Git fetch does not publish symbolic or unborn remote branch mappings; correct the remote and retry",
            ));
        }
        if repo
            .try_find_reference(local.as_bstr())
            .map_err(|_| failure("cannot inspect Git tracking reference; correct its metadata and retry fetch"))?
            .is_some_and(|reference| reference.try_id().is_none())
        {
            return Err(failure(
                "Git fetch refuses a symbolic tracking destination; preserve its local target and explicitly correct the tracking ref",
            ));
        }
    }
    Ok(())
}

fn validate_outcome(repo: &gix::Repository, outcome: &Outcome) -> Result<(), GitError> {
    let updates = match &outcome.status {
        Status::Change { update_refs, .. } => update_refs,
        Status::NoPackReceived {
            dry_run: false,
            update_refs,
            ..
        } => update_refs,
        Status::NoPackReceived { dry_run: true, .. } => return Err(failure("Git fetch did not publish: the SDK reported a dry run")),
    };
    if updates.updates.len() != outcome.ref_map.mappings.len()
        || updates.updates.iter().any(|update| {
            !matches!(update.mode, Mode::New | Mode::FastForward | Mode::NoChangeNeeded | Mode::Forced) || update.type_change.is_some()
        })
        || updates
            .edits
            .iter()
            .any(|edit| !edit.name.as_bstr().starts_with(b"refs/remotes/origin/") || edit.deref)
    {
        return Err(failure(
            "Git fetch rejected one or more tracking updates; downloaded/published state may be partial; inspect refs and explicitly retry",
        ));
    }
    // A top-level Ok is not proof that each advertised destination was published.
    for mapping in &outcome.ref_map.mappings {
        let local = mapping
            .local
            .as_ref()
            .ok_or_else(|| failure("Git fetch did not provide a tracking destination"))?;
        let actual = repo
            .try_find_reference(local.as_bstr())
            .map_err(|_| failure("cannot verify published tracking refs; inspect retained state and explicitly retry fetch"))?;
        if actual.as_ref().and_then(|reference| reference.try_id()).as_deref() != mapping.remote.as_id() {
            return Err(failure(
                "Git tracking publication is incomplete or changed concurrently; inspect refs and explicitly retry fetch",
            ));
        }
    }
    Ok(())
}
