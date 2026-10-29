//! Opaque private SDK index and continuously held real-index publication.

use std::{fs, io::Write, path::Path};

use crate::GitError;

pub(super) fn load(real: &Path, private: &Path, born: bool) -> Result<git2::Index, GitError> {
    let exists = match fs::read(real) {
        Ok(bytes) => {
            fs::write(private, bytes).map_err(|error| GitError::Git(format!("cannot copy private checkout index: {error}")))?;
            true
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound && !born => false,
        Err(error) => {
            return Err(GitError::Git(format!(
                "cannot read checkout index: {error}; a born HEAD requires its existing index to preserve staged selections"
            )));
        }
    };
    let mut index = git2::Index::open(private)
        .map_err(|_| GitError::Git("cannot decode checkout index; explicitly repair it before retrying".into()))?;
    if !exists {
        // Only the SDK generates an empty index; no index encoding lives here.
        index
            .write()
            .map_err(|_| GitError::Git("cannot write private empty checkout index".into()))?;
    }
    if index.has_conflicts() {
        return Err(GitError::Git(
            "checkout refuses an unmerged index; resolve the conflicts before retrying".into(),
        ));
    }
    if index.iter().any(|entry| entry.mode == 0o160000) {
        return Err(GitError::Git("checkout does not support index gitlinks/submodules".into()));
    }
    Ok(index)
}

pub(super) fn reject_staged_overlap(
    repo: &git2::Repository,
    index: &git2::Index,
    baseline: Option<&git2::Tree<'_>>,
    target: &git2::Tree<'_>,
) -> Result<(), GitError> {
    let staged = repo
        .diff_tree_to_index(baseline, Some(index), None)
        .map_err(|_| GitError::Git("cannot compare staged checkout index selections".into()))?;
    let changed = repo
        .diff_tree_to_tree(baseline, Some(target), None)
        .map_err(|_| GitError::Git("cannot compare checkout baseline and target trees".into()))?;
    for staged_delta in staged.deltas() {
        for staged_file in [staged_delta.old_file(), staged_delta.new_file()] {
            let Some(staged_path) = staged_file.path_bytes() else {
                continue;
            };
            for changed_delta in changed.deltas() {
                for changed_file in [changed_delta.old_file(), changed_delta.new_file()] {
                    if changed_file.path_bytes().is_some_and(|path| overlaps(staged_path, path)) {
                        return Err(GitError::Git(
                            "checkout would overwrite an overlapping staged selection; preserve or explicitly resolve it before retrying"
                                .into(),
                        ));
                    }
                }
            }
        }
    }
    Ok(())
}

fn overlaps(one: &[u8], two: &[u8]) -> bool {
    one == two
        || one.strip_prefix(two).is_some_and(|suffix| suffix.starts_with(b"/"))
        || two.strip_prefix(one).is_some_and(|suffix| suffix.starts_with(b"/"))
}

pub(super) fn prepare(lock: &mut gix::lock::File, private: &Path) -> Result<(), GitError> {
    // Checkout itself wrote this file. Never reconstruct it from a tree or
    // rewrite a stale in-memory index, which could consume unrelated staging.
    let bytes = fs::read(private).map_err(|_| super::worktree::partial("cannot read the SDK-written private index"))?;
    lock.write_all(&bytes)
        .and_then(|()| lock.with_mut(|file| file.sync_all()))
        .map_err(|_| super::worktree::partial("cannot prepare final index bytes; HEAD has not been published"))
}
