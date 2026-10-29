//! Guarded local branch creation and read-only explicit-commit ancestry.

use std::collections::HashSet;

use gix::refs::{
    FullName,
    transaction::{PreviousValue, RefEdit},
};

use crate::commit::{validate_branch_reference, validate_signature};
use crate::{GitError, ProjectRepo};

impl ProjectRepo {
    /// Create an unused full local branch at an available commit with a supplied actor.
    ///
    /// The reference must be under `refs/heads/` and absent, even if an existing
    /// branch has the same target. Tags are not peeled. The supplied actor is
    /// used unchanged for the reflog; no configured identity is required.
    ///
    /// HEAD, index, worktree, config and other refs are preserved, including in
    /// bare repositories and with detached or unborn HEAD. Locks and competing
    /// creation fail without retry; no hooks, filters or network are invoked.
    pub fn create_branch(&self, reference: &str, target: gix::ObjectId, committer: gix::actor::SignatureRef<'_>) -> Result<(), GitError> {
        let reference = validate_branch_reference(reference)?;
        let actor = committer
            .to_owned()
            .map_err(|_| GitError::Git("cannot parse branch reflog actor timestamp".into()))?;
        validate_signature(&actor)?;
        committer
            .write_to(&mut std::io::sink())
            .map_err(|error| GitError::Git(format!("cannot serialize branch reflog actor: {error:#}")))?;
        // The SDK's header-time parser is deliberately lenient. Require a
        // lossless round trip instead of accepting an invalid zone as UTC.
        let mut time = gix::date::parse::TimeBuf::default();
        if actor.to_ref(&mut time).time != committer.time {
            return Err(GitError::Git(
                "branch reflog actor requires a raw timestamp and timezone that round-trip without normalization".into(),
            ));
        }

        let repo = gix::open_opts(
            self.repo.git_dir(),
            gix::open::Options::default().strict_config(true).open_path_as_is(true),
        )
        .map_err(|error| GitError::Git(format!("cannot reopen repository for branch creation: {error:#}")))?;
        repo.find_commit(target)
            .map_err(|error| GitError::Git(format!("cannot read branch target commit {target}: {error:#}")))?
            .decode()
            .map_err(|error| GitError::Git(format!("cannot decode branch target commit {target}: {error:#}")))?;

        // Deny observed duplicates before locks, including packed and symbolic
        // refs. Atomic publication below still checks absence for racing writers.
        if repo
            .try_find_reference(&reference)
            .map_err(|error| GitError::Git(format!("cannot inspect branch {reference}: {error:#}")))?
            .is_some()
        {
            return Err(GitError::Git(format!("cannot create branch {reference}: it already exists")));
        }
        publish_branch(&repo, reference, target, committer)
    }

    /// Find one SDK-selected best common ancestor of two available commits.
    ///
    /// Both inputs and their reachable commits must be available and decodable,
    /// including for equal inputs. Tags are not peeled. Only genuinely unrelated
    /// histories return `None`; missing or corrupt ancestry returns an error.
    /// Multiple best bases follow the SDK's selection, not a uniqueness rule.
    /// No refs, index, worktree files or config are changed; HEAD is not consulted.
    pub fn merge_base(&self, one: gix::ObjectId, two: gix::ObjectId) -> Result<Option<gix::ObjectId>, GitError> {
        let repo = gix::open_opts(
            self.repo.git_dir(),
            gix::open::Options::default().strict_config(true).open_path_as_is(true),
        )
        .map_err(|error| GitError::Git(format!("cannot reopen repository for merge-base: {error:#}")))?;
        validate_commit_graph(&repo, one, two)?;
        repo.merge_base(one, two)
            .map(|base| base.map(|id| id.detach()))
            .map_err(|error| GitError::Git(format!("cannot find merge-base for {one} and {two}: {error:#}")))
    }
}

fn publish_branch(
    repo: &gix::Repository,
    reference: FullName,
    target: gix::ObjectId,
    committer: gix::actor::SignatureRef<'_>,
) -> Result<(), GitError> {
    let edit = RefEdit::update(
        reference.clone(),
        target,
        PreviousValue::MustNotExist,
        format!("branch: Created from {target}"),
    )
    .with_deref(false);
    let transaction = repo
        .refs
        .transaction()
        .prepare([edit], gix::lock::acquire::Fail::Immediately, gix::lock::acquire::Fail::Immediately)
        .map_err(|error| GitError::Git(format!("cannot lock or prepare branch {reference}: {error:#}")))?;
    let edits = transaction
        .commit(Some(committer))
        .map_err(|error| GitError::Git(format!("cannot create branch {reference}: {error:#}")))?;
    // As in publish_commit, the pinned SDK accepts an identical existing OID as
    // a no-op under MustNotExist. Its returned previous value distinguishes that
    // race from creation; no ref or reflog was changed by the losing request.
    if edits.iter().any(|edit| edit.change.previous_value().is_some()) {
        return Err(GitError::Git(format!("cannot create branch {reference}: it already exists")));
    }
    Ok(())
}

fn validate_commit_graph(repo: &gix::Repository, one: gix::ObjectId, two: gix::ObjectId) -> Result<(), GitError> {
    // SDK merge-base traversal may skip missing parents and short-circuit equal
    // inputs. Validate the actual objects first, not just cached graph entries,
    // so incomplete history cannot become a successful base or false absence.
    // This walk validates only; the SDK remains responsible for base selection.
    let mut pending = vec![one, two];
    let mut seen = HashSet::new();
    while let Some(id) = pending.pop() {
        if !seen.insert(id) {
            continue;
        }
        let commit = repo
            .find_commit(id)
            .map_err(|error| GitError::Git(format!("cannot read merge-base graph commit {id}: {error:#}")))?;
        let decoded = commit
            .decode()
            .map_err(|error| GitError::Git(format!("cannot decode merge-base graph commit {id}: {error:#}")))?;
        pending.extend(decoded.parents());
    }
    Ok(())
}
