//! SDK safe checks and a latched prewrite conflict veto.

use crate::GitError;

pub(super) fn checkout(repo: &git2::Repository, target: &git2::Object<'_>) -> Result<(), GitError> {
    let mut vetoed = false;
    let result = {
        let mut options = git2::build::CheckoutBuilder::new();
        options
            .safe()
            .overwrite_ignored(false)
            .allow_conflicts(false)
            .remove_untracked(false)
            .remove_ignored(false)
            .recreate_missing(false)
            .update_index(true)
            .refresh(true)
            .notify_on(git2::CheckoutNotificationType::CONFLICT)
            .notify(|why, _, _, _, _| {
                vetoed |= why.contains(git2::CheckoutNotificationType::CONFLICT);
                !vetoed
            });
        #[cfg(test)]
        options.progress(|_, _, _| super::observer::visit(super::observer::Phase::SdkProgress));
        repo.checkout_tree(target, Some(&mut options))
    };
    #[cfg(test)]
    if let Err(error) = &result {
        super::observer::visit(super::observer::Phase::SdkFailed {
            code: error.code(),
            class: error.class(),
        });
    }
    #[cfg(test)]
    super::observer::visit(super::observer::Phase::SdkReturned {
        vetoed,
        succeeded: result.is_ok(),
    });
    // The pinned binding may return Ok after a native notification veto.
    // That is not permission to publish either HEAD or the real index.
    if vetoed {
        return Err(GitError::Git(
            "checkout conflict would overwrite local content; HEAD and real index were not published; preserve the edits before retrying"
                .into(),
        ));
    }
    result.map_err(|_| partial("SDK safe checkout failed (conflict, obstruction or filesystem error); HEAD has not been published"))
}

pub(super) fn partial(reason: &str) -> GitError {
    GitError::Git(format!(
        "checkout partial state: {reason}; worktree changes may already exist, real index was not published; \
         no user files were restored; inspect HEAD, index and worktree, explicitly correct the obstruction/state and retry"
    ))
}
