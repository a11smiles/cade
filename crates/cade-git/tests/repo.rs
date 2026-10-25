//! Integration tests for `cade_git::ProjectRepo` against real repositories in
//! temp dirs. Fixtures are built with the `git` CLI with global and system
//! config disabled, so the developer's `~/.gitconfig` is never read.

use cade_git::{GitError, ProjectRepo, run_commit_message};
use std::path::Path;
use std::process::Command;

fn git(dir: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args([
            "-c",
            "user.name=CADE Test",
            "-c",
            "user.email=test@example.invalid",
            "-c",
            "commit.gpgsign=false",
        ])
        .args(args)
        .current_dir(dir)
        .env("GIT_CONFIG_GLOBAL", if cfg!(windows) { "NUL" } else { "/dev/null" })
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .output()
        .expect("git is installed");
    assert!(out.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
}

#[test]
fn project_repo_open_plain_directory_returns_not_a_repo_with_path() {
    let dir = tempfile::tempdir().unwrap();
    match ProjectRepo::open(dir.path()) {
        Err(GitError::NotARepo(p)) => assert_eq!(p, dir.path()),
        other => panic!("expected NotARepo, got {:?}", other.err()),
    }
}

#[test]
fn project_repo_open_missing_directory_returns_not_a_repo() {
    let dir = tempfile::tempdir().unwrap();
    let missing = dir.path().join("does-not-exist");
    assert!(matches!(ProjectRepo::open(&missing), Err(GitError::NotARepo(p)) if p == missing));
}

#[test]
fn project_repo_init_new_directory_has_workdir_and_no_commits() {
    let dir = tempfile::tempdir().unwrap();
    let r = ProjectRepo::init(dir.path()).unwrap();
    assert!(!r.has_commits());
    assert_eq!(r.workdir().unwrap().canonicalize().unwrap(), dir.path().canonicalize().unwrap());
    assert!(dir.path().join(".git").is_dir());
}

#[test]
fn project_repo_init_existing_repo_returns_git_error() {
    let dir = tempfile::tempdir().unwrap();
    ProjectRepo::init(dir.path()).unwrap();
    assert!(matches!(ProjectRepo::init(dir.path()), Err(GitError::Git(_))));
}

#[test]
fn project_repo_branch_named_branch_returns_short_name() {
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q", "-b", "cade/intake-flow"]);
    let r = ProjectRepo::open(dir.path()).unwrap();
    assert_eq!(r.branch().unwrap().as_deref(), Some("cade/intake-flow"));
}

#[test]
fn project_repo_has_commits_after_first_commit_returns_true() {
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q", "-b", "main"]);
    git(dir.path(), &["commit", "-q", "--allow-empty", "-m", "Save"]);
    assert!(ProjectRepo::open(dir.path()).unwrap().has_commits());
}

#[test]
fn project_repo_branch_detached_head_returns_none() {
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q", "-b", "main"]);
    git(dir.path(), &["commit", "-q", "--allow-empty", "-m", "Save"]);
    git(dir.path(), &["checkout", "-q", "--detach"]);
    let r = ProjectRepo::open(dir.path()).unwrap();
    assert_eq!(r.branch().unwrap(), None);
    assert!(r.has_commits());
}

#[test]
fn project_repo_open_bare_repo_has_no_workdir() {
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q", "--bare", "-b", "main"]);
    assert!(ProjectRepo::open(dir.path()).unwrap().workdir().is_none());
}

#[test]
fn run_commit_message_committed_with_git_is_parsed_as_trailers() {
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q", "-b", "main"]);
    let msg = run_commit_message("Built Step 1", "run-42", "gpt-5", 0);
    git(dir.path(), &["commit", "-q", "--allow-empty", "-m", &msg]);
    let out = Command::new("git")
        .args(["log", "-1", "--format=%(trailers:only,unfold)"])
        .current_dir(dir.path())
        .env("GIT_CONFIG_GLOBAL", if cfg!(windows) { "NUL" } else { "/dev/null" })
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .output()
        .unwrap();
    assert_eq!(
        String::from_utf8(out.stdout).unwrap().trim_end(),
        "CADE-Copilot-Run: run-42\nCADE-Model: gpt-5\nCADE-Tokens: 0"
    );
}
