//! Safe existing-worktree switching with SDK-owned indexes and reference guards.

use crate::{GitError, ProjectRepo};

mod index;
mod references;
mod validate;
mod worktree;

// These filesystem integration probes need private access, not an exported
// hook or a second checkout implementation. Normal integration tests use the API.
#[cfg(test)]
#[path = "../tests/checkout_boundaries/mod.rs"]
mod boundaries;
#[cfg(test)]
mod observer;

impl ProjectRepo {
    /// Switch to a full local branch using the supplied reflog actor.
    ///
    /// Safe local edits outside the branch changes are preserved. Overlapping
    /// staged selections, conflicts, executable filters, gitlinks, nested
    /// repositories and unresolved operations are rejected without discarding data.
    ///
    /// HEAD is published before the real index, with current/target branch
    /// guards held throughout. This is not a globally atomic transaction: late
    /// failures report possible partial state and require explicit inspection,
    /// correction and retry. User files are never automatically restored.
    pub fn checkout(&self, reference: &str, committer: gix::actor::SignatureRef<'_>) -> Result<(), GitError> {
        crate::commit::validate_branch_reference(reference)?;
        let actor = validate::actor(committer)?;
        // Declare the temporary directory first: repository/index handles must
        // release their files before its destructor runs, including on Windows.
        let private = tempfile::tempdir().map_err(crate::ge)?;
        let repo = validate::reopen(self.repo.git_dir())?;
        validate::repository(&repo)?;
        let observed = references::observe(&repo, reference)?;
        if !references::is_current(&observed, reference) {
            references::reject_other_worktree(&repo, reference)?;
        }
        #[cfg(test)]
        observer::visit(observer::Phase::Observed);
        let target = references::target(&observed, reference)?;
        let baseline = references::baseline(&observed)?;
        validate::commit_tree(&repo, target)?;
        if let Some(baseline) = baseline {
            validate::commit_tree(&repo, baseline)?;
        }

        let index_path = repo.index_path();
        validate::regular_file(&index_path, "index")?;
        let mut index_lock = gix::lock::File::acquire_to_update_resource(&index_path, gix::lock::acquire::Fail::Immediately, None, 0)
            .map_err(|error| GitError::Git(format!("cannot lock checkout index: {error}")))?;
        let sdk = git2::Repository::open(repo.git_dir())
            .map_err(|_| GitError::Git("cannot reopen checkout repository metadata/configuration".into()))?;
        validate::sdk_filters(&sdk)?;
        let (mut head, branches) = references::lock(&sdk, &observed)?;
        references::revalidate(&repo, reference, &observed)?;
        #[cfg(test)]
        observer::visit(observer::Phase::Guarded);

        let private_path = private.path().join("index");
        let mut private_index = index::load(&index_path, &private_path, baseline.is_some())?;
        sdk.set_index(&mut private_index)
            .map_err(|_| GitError::Git("cannot attach private checkout index".into()))?;
        let target = sdk
            .find_commit(git2::Oid::from_bytes(target.as_slice()).map_err(crate::ge)?)
            .map_err(|_| GitError::Git("cannot read checkout target commit object".into()))?;
        let baseline = baseline
            .map(|id| {
                sdk.find_commit(git2::Oid::from_bytes(id.as_slice()).map_err(crate::ge)?)
                    .and_then(|commit| commit.tree())
                    .map_err(|_| GitError::Git("cannot read checkout baseline tree object".into()))
            })
            .transpose()?;
        let target_tree = target
            .tree()
            .map_err(|_| GitError::Git("cannot read checkout target tree object".into()))?;
        index::reject_staged_overlap(&sdk, &private_index, baseline.as_ref(), &target_tree)?;

        if references::is_current(&observed, reference) {
            return Ok(());
        }
        // Rescan while the target guard is held, immediately before native
        // mutation. External HEAD writers need not honor this guard: this is
        // detection of observed occupancy, not global worktree atomicity.
        references::reject_other_worktree(&repo, reference)?;
        worktree::checkout(&sdk, target.as_object())?;
        #[cfg(test)]
        observer::visit(observer::Phase::WorktreeUpdated);
        index::prepare(&mut index_lock, &private_path)?;
        head.set_symbolic_target("HEAD", reference, Some(&actor), "checkout: switch local branch")
            .map_err(|_| worktree::partial("cannot prepare HEAD publication; HEAD has not been published"))?;
        head.commit()
            .map_err(|_| worktree::partial("HEAD/reflog publication failed and may be partial"))?;
        #[cfg(test)]
        observer::visit(observer::Phase::HeadPublished);
        // HEAD's publishing transaction is consumed, but the SEPARATE no-edit
        // transaction continues excluding current/target branch writers.
        index_lock.commit().map_err(|error| {
            GitError::Git(format!(
                "checkout partial state: HEAD was published and the worktree updated, but final index publication failed: {error}; \
                 no user files were restored; inspect HEAD, index and worktree, correct the obstruction/state and explicitly retry"
            ))
        })?;
        drop(branches);
        Ok(())
    }
}
