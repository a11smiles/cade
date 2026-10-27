//! No-edit current/target guards, distinct from the consuming HEAD publisher.

use std::collections::BTreeMap;

use gix::refs::Target;

use crate::{GitError, commit::validate_branch_reference};

pub(super) fn observe(repo: &gix::Repository, target: &str) -> Result<BTreeMap<String, Option<Target>>, GitError> {
    let head = read(repo, "HEAD")?.ok_or_else(|| GitError::Git("checkout HEAD reference is missing".into()))?;
    let mut names = vec!["HEAD".to_owned(), target.to_owned()];
    if let Target::Symbolic(current) = &head {
        let current = std::str::from_utf8(current.as_bstr())
            .map_err(|_| GitError::Git("checkout current branch reference cannot be represented without conversion".into()))?;
        validate_branch_reference(current)?;
        names.push(current.to_owned());
    }
    names.sort();
    names.dedup();
    names.into_iter().map(|name| read(repo, &name).map(|value| (name, value))).collect()
}

fn read(repo: &gix::Repository, name: &str) -> Result<Option<Target>, GitError> {
    repo.try_find_reference(name)
        .map(|reference| reference.map(|reference| reference.detach().target))
        .map_err(|_| GitError::Git("cannot inspect checkout branch/HEAD reference".into()))
}

pub(super) fn target(observed: &BTreeMap<String, Option<Target>>, name: &str) -> Result<gix::ObjectId, GitError> {
    match observed.get(name).and_then(Option::as_ref) {
        Some(Target::Object(id)) => Ok(*id),
        Some(Target::Symbolic(_)) => Err(GitError::Git(
            "checkout requires a direct branch target, not a symbolic reference".into(),
        )),
        None => Err(GitError::Git(
            "checkout target branch is missing; select an existing local branch".into(),
        )),
    }
}

pub(super) fn baseline(observed: &BTreeMap<String, Option<Target>>) -> Result<Option<gix::ObjectId>, GitError> {
    match observed.get("HEAD").and_then(Option::as_ref) {
        Some(Target::Object(id)) => Ok(Some(*id)),
        Some(Target::Symbolic(name)) => {
            let name =
                std::str::from_utf8(name.as_bstr()).map_err(|_| GitError::Git("checkout HEAD branch name is not representable".into()))?;
            match observed.get(name).and_then(Option::as_ref) {
                None => Ok(None),
                Some(Target::Object(id)) => Ok(Some(*id)),
                Some(Target::Symbolic(_)) => Err(GitError::Git("checkout baseline requires a direct current branch reference".into())),
            }
        }
        None => Err(GitError::Git("checkout HEAD reference is missing".into())),
    }
}

pub(super) fn is_current(observed: &BTreeMap<String, Option<Target>>, target: &str) -> bool {
    matches!(
        observed.get("HEAD").and_then(Option::as_ref),
        Some(Target::Symbolic(name)) if name.as_bstr() == target.as_bytes()
    )
}

pub(super) fn reject_other_worktree(repo: &gix::Repository, target: &str) -> Result<(), GitError> {
    let inspection_error =
        || GitError::Git("cannot inspect worktree registrations/HEAD; repair repository metadata and explicitly retry".into());
    // Compare metadata identities, not user-file contents or HEAD object IDs.
    // Canonicalization also excludes our own registration through path aliases.
    let own = repo
        .current_dir()
        .join(repo.git_dir())
        .canonicalize()
        .map_err(|_| inspection_error())?;
    for other in repo.worktrees_including_main().map_err(|_| inspection_error())? {
        let other = other.map_err(|_| inspection_error())?;
        // A bare main's HEAD is not a checkout. Do not use is_bare(): linked
        // worktrees can inherit core.bare while still having a working tree.
        if other.workdir().is_none() {
            continue;
        }
        if other.git_dir().canonicalize().map_err(|_| inspection_error())? == own {
            continue;
        }
        let head = read(&other, "HEAD").map_err(|_| inspection_error())?.ok_or_else(inspection_error)?;
        if matches!(head, Target::Symbolic(name) if name.as_bstr() == target.as_bytes()) {
            return Err(GitError::Git(
                "checkout target branch is checked out in another worktree; switch or detach that worktree before explicitly retrying"
                    .into(),
            ));
        }
    }
    Ok(())
}

pub(super) fn lock<'repo>(
    repo: &'repo git2::Repository,
    observed: &BTreeMap<String, Option<Target>>,
) -> Result<(git2::Transaction<'repo>, git2::Transaction<'repo>), GitError> {
    let mut head = repo
        .transaction()
        .map_err(|_| GitError::Git("cannot prepare checkout HEAD lock transaction".into()))?;
    head.lock_ref("HEAD")
        .map_err(|_| GitError::Git("cannot lock checkout HEAD; release the owning operation before explicitly retrying".into()))?;
    let mut branches = repo
        .transaction()
        .map_err(|_| GitError::Git("cannot prepare checkout branch lock transaction".into()))?;
    for name in observed.keys().filter(|name| name.as_str() != "HEAD") {
        branches.lock_ref(name).map_err(|_| {
            GitError::Git("cannot lock checkout current/target branch; release the owning operation before retrying".into())
        })?;
    }
    Ok((head, branches))
}

pub(super) fn revalidate(repo: &gix::Repository, target: &str, expected: &BTreeMap<String, Option<Target>>) -> Result<(), GitError> {
    let fresh = super::validate::reopen(repo.git_dir())?;
    if observe(&fresh, target)? != *expected {
        return Err(GitError::Git(
            "checkout HEAD/current/target reference changed before locks were acquired; inspect the new state and explicitly retry".into(),
        ));
    }
    Ok(())
}
