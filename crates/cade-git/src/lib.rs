//! Git persistence for CADE projects.
//!
//! Every CADE project is a git repository. A save is a commit, a Copilot
//! run is a commit tagged with its run id, and History is the commit log.
//! This crate uses gitoxide, with a checkout-only git2 fallback; no git binary
//! is required by production operations.
//!
//! Implemented here: open/init, reading HEAD, read-only local status, commits
//! from explicitly supplied SDK trees, parents and metadata, local branch
//! creation, explicit-commit merge-base queries and safe local checkout.
//! Status rejects executable clean/process filters and unsupported submodule
//! or nested-repository inspection. Save/run integration and
//! restore-as-new-commit remain CORE/HIST backlog items.

use std::path::{Path, PathBuf};

#[cfg(test)]
use checkout::boundaries::support;

mod branches;
#[cfg_attr(test, macro_use)]
mod checkout;
mod commit;
mod transport;

#[derive(Debug, thiserror::Error)]
pub enum GitError {
    #[error("not a CADE project repository: {0}")]
    NotARepo(PathBuf),
    #[error("git: {0}")]
    Git(String),
}

fn ge<E: std::fmt::Display>(e: E) -> GitError {
    GitError::Git(e.to_string())
}

fn path_entry_exists(path: &Path) -> Result<bool, GitError> {
    match std::fs::symlink_metadata(path) {
        Ok(_) => Ok(true),
        Err(error) if matches!(error.kind(), std::io::ErrorKind::NotFound | std::io::ErrorKind::NotADirectory) => Ok(false),
        Err(error) => Err(GitError::Git(format!("cannot inspect {}: {error}", path.display()))),
    }
}

fn has_repository_metadata(path: &Path) -> Result<bool, GitError> {
    // symlink_metadata also detects occupied dangling .git symlinks.
    Ok(path_entry_exists(&path.join(".git"))? || (path_entry_exists(&path.join("objects"))? && path_entry_exists(&path.join("refs"))?))
}

fn reject_executable_filters(repo: &gix::Repository) -> Result<(), GitError> {
    let config = repo.config_snapshot();
    for section in config.plumbing().sections_by_name("filter").into_iter().flatten() {
        let body = section.body();
        if body.value_implicit("clean").is_some() || body.value_implicit("process").is_some() {
            return Err(GitError::Git(
                "local status does not support configured clean/process filters; remove the executable filter configuration and retry"
                    .into(),
            ));
        }
    }
    Ok(())
}

/// Trailer added to commits made by an applied Copilot run.
pub const RUN_TRAILER: &str = "CADE-Copilot-Run";

pub struct ProjectRepo {
    repo: gix::Repository,
}

impl ProjectRepo {
    /// Open an existing repository, including bare repositories, without changing it.
    pub fn open(path: &Path) -> Result<Self, GitError> {
        let repo = match gix::open_opts(path, gix::open::Options::default().strict_config(true)) {
            Ok(repo) => repo,
            Err(error) => {
                if !error.is_not_found() || has_repository_metadata(path)? {
                    return Err(ge(error));
                }
                return Err(GitError::NotARepo(path.to_path_buf()));
            }
        };
        // Opening discovers metadata but does not validate HEAD's reference syntax.
        repo.head().map_err(ge)?;
        Ok(ProjectRepo { repo })
    }

    /// Initialize a new working repository, preserving unrelated files.
    ///
    /// Existing repositories and occupied metadata must be opened, not reinitialized.
    pub fn init(path: &Path) -> Result<Self, GitError> {
        if has_repository_metadata(path)? {
            return Err(GitError::Git(
                "repository metadata already exists; use ProjectRepo::open instead of initializing over it".into(),
            ));
        }
        let repo = gix::init(path).map_err(ge)?;
        Ok(ProjectRepo { repo })
    }

    /// Read HEAD/index/worktree changes without modifying the project.
    ///
    /// Returns separate SDK events in unspecified order, retaining relative path
    /// bytes and individual untracked files. Bare repositories, executable
    /// clean/process filters, submodules and nested repositories are unsupported
    /// and return errors rather than an incomplete or falsely clean result.
    pub fn status(&self) -> Result<Vec<gix::status::Item>, GitError> {
        // Reopening avoids stale configuration/index caches, including after a
        // failed request whose metadata was subsequently corrected by the caller.
        let repo = gix::open_opts(
            self.repo.git_dir(),
            gix::open::Options::default().strict_config(true).open_path_as_is(true),
        )
        .map_err(ge)?;
        if repo.workdir().is_none() {
            return Err(GitError::Git(
                "local status requires a working tree; bare repositories are unsupported".into(),
            ));
        }
        reject_executable_filters(&repo)?;
        let index = repo.index_or_load_from_head_or_empty().map_err(ge)?;
        if index.entries().iter().any(|entry| entry.mode.is_submodule()) {
            return Err(GitError::Git(
                "local status does not support submodule inspection; use a repository without gitlinks".into(),
            ));
        }

        let dirwalk = repo
            .dirwalk_options()
            .map_err(ge)?
            .recurse_repositories(false)
            .classify_untracked_bare_repositories(true);
        let changes = repo
            .status(gix::progress::Discard)
            .map_err(ge)?
            .index(index)
            // untracked_files alone cannot restore a config-disabled dirwalk.
            .index_worktree_options_mut(|options| options.dirwalk_options = Some(dirwalk))
            .untracked_files(gix::status::UntrackedFiles::Files)
            .index_worktree_submodules(None)
            .tree_index_track_renames(gix::status::tree_index::TrackRenames::Disabled)
            .into_iter(Vec::<gix::bstr::BString>::new())
            .map_err(ge)?;
        let mut items = Vec::new();
        for change in changes {
            let item = change.map_err(ge)?;
            if matches!(
                &item,
                gix::status::Item::IndexWorktree(gix::status::index_worktree::Item::DirectoryContents { entry, .. })
                    if entry.disk_kind == Some(gix::dir::entry::Kind::Repository)
            ) {
                return Err(GitError::Git(
                    "local status does not support nested repositories; inspect the nested repository separately".into(),
                ));
            }
            items.push(item);
        }
        // Never apply the iterator outcome's index stat-cache updates.
        Ok(items)
    }

    /// Short branch name of HEAD, e.g. `cade/intake-flow`. `None` when detached.
    pub fn branch(&self) -> Result<Option<String>, GitError> {
        Ok(self.repo.head_name().map_err(ge)?.map(|n| n.shorten().to_string()))
    }

    /// Whether HEAD points at a commit yet.
    pub fn has_commits(&self) -> bool {
        self.repo.head_id().is_ok()
    }

    pub fn workdir(&self) -> Option<&Path> {
        self.repo.workdir()
    }
}

/// Commit message for an applied Copilot run.
pub fn run_commit_message(summary: &str, run_id: &str, model: &str, tokens: u64) -> String {
    format!("{summary}\n\nApplied from Copilot in CADE.\n\n{RUN_TRAILER}: {run_id}\nCADE-Model: {model}\nCADE-Tokens: {tokens}\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_then_open() {
        let dir = std::env::temp_dir().join(format!("cade-git-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let r = ProjectRepo::init(&dir).unwrap();
        assert!(!r.has_commits());
        assert!(r.branch().unwrap().is_some());
        assert!(ProjectRepo::open(&dir).is_ok());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn run_message_has_trailers() {
        let m = run_commit_message("Added error states to Step 1", "r7", "Claude Sonnet 5.5", 186_000);
        assert!(m.contains("CADE-Copilot-Run: r7"));
    }

    #[test]
    fn run_commit_message_applied_run_matches_trailer_format() {
        let m = run_commit_message("Added error states to Step 1", "r7", "Claude Sonnet 5.5", 186_000);
        assert_eq!(
            m,
            "Added error states to Step 1\n\nApplied from Copilot in CADE.\n\n\
             CADE-Copilot-Run: r7\nCADE-Model: Claude Sonnet 5.5\nCADE-Tokens: 186000\n"
        );
        assert!(m.starts_with("Added error states to Step 1\n\n"));
        assert!(m.contains(&format!("{RUN_TRAILER}: r7\n")));
    }
}
