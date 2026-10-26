//! Git persistence for CADE projects.
//!
//! Every CADE project is a git repository. A save is a commit, a Copilot
//! run is a commit tagged with its run id, and History is the commit log.
//! This crate uses gitoxide (pure Rust), so no git binary is required.
//!
//! Implemented here: open/init and reading HEAD. Commit-per-save, run
//! trailers, branching and restore-as-new-commit are CORE/HIST backlog items.

use std::path::{Path, PathBuf};

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

/// Trailer added to commits made by an applied Copilot run.
pub const RUN_TRAILER: &str = "CADE-Copilot-Run";

pub struct ProjectRepo {
    repo: gix::Repository,
}

impl ProjectRepo {
    pub fn open(path: &Path) -> Result<Self, GitError> {
        let repo = gix::open(path).map_err(|_| GitError::NotARepo(path.to_path_buf()))?;
        Ok(ProjectRepo { repo })
    }

    pub fn init(path: &Path) -> Result<Self, GitError> {
        let repo = gix::init(path).map_err(ge)?;
        Ok(ProjectRepo { repo })
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
