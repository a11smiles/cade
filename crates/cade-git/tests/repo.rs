//! Integration tests for `cade_git::ProjectRepo` against real repositories in
//! temp dirs. Fixtures are built with the `git` CLI with global and system
//! config disabled, so the developer's `~/.gitconfig` is never read.

use cade_git::{GitError, ProjectRepo, run_commit_message};
use std::path::Path;
use std::process::Command;

#[path = "repo/commit.rs"]
mod commit;
#[path = "repo/local.rs"]
mod local;
#[path = "repo/status.rs"]
mod status;

fn git(dir: &Path, args: &[&str]) -> Vec<u8> {
    let out = Command::new("git")
        .env_clear()
        .envs(std::env::vars_os().filter(|(key, _)| matches!(key.to_str(), Some("PATH" | "SystemRoot" | "SYSTEMROOT" | "TEMP" | "TMP"))))
        .args([
            "-c",
            "user.name=CADE Test",
            "-c",
            "user.email=test@example.invalid",
            "-c",
            "commit.gpgsign=false",
            "-c",
            "init.templateDir=",
            "-c",
            "core.autocrlf=false",
        ])
        .args(args)
        .current_dir(dir)
        .env("GIT_CONFIG_GLOBAL", if cfg!(windows) { "NUL" } else { "/dev/null" })
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_ATTR_NOSYSTEM", "1")
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("HOME", dir)
        .env("USERPROFILE", dir)
        .env("XDG_CONFIG_HOME", dir)
        .env("GIT_AUTHOR_DATE", "2000-01-01T00:00:00+0000")
        .env("GIT_COMMITTER_DATE", "2000-01-01T00:00:00+0000")
        .output()
        .expect("git is installed");
    assert!(out.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
    out.stdout
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
    let before = snapshot(dir.path());
    assert!(matches!(ProjectRepo::init(dir.path()), Err(GitError::Git(_))));
    assert_eq!(snapshot(dir.path()), before);
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

fn committed_repo() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q", "-b", "main"]);
    std::fs::write(dir.path().join("tracked.txt"), b"original\n").unwrap();
    git(dir.path(), &["add", "--", "tracked.txt"]);
    git(dir.path(), &["commit", "-q", "-m", "Fixture"]);
    dir
}

// Include empty directories and symlink targets without following symlinks.
// Comparing the whole fixture also catches lock files, config/credential writes,
// changed objects/refs, and index stat-cache writes.
fn snapshot(root: &Path) -> std::collections::BTreeMap<std::path::PathBuf, (std::fs::FileType, Vec<u8>)> {
    fn visit(root: &Path, dir: &Path, entries: &mut std::collections::BTreeMap<std::path::PathBuf, (std::fs::FileType, Vec<u8>)>) {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            let kind = std::fs::symlink_metadata(&path).unwrap().file_type();
            let bytes = if kind.is_symlink() {
                std::fs::read_link(&path).unwrap().as_os_str().as_encoded_bytes().to_vec()
            } else if kind.is_file() {
                std::fs::read(&path).unwrap()
            } else {
                Vec::new()
            };
            entries.insert(path.strip_prefix(root).unwrap().to_path_buf(), (kind, bytes));
            if kind.is_dir() {
                visit(root, &path, entries);
            }
        }
    }
    let mut entries = std::collections::BTreeMap::new();
    visit(root, root, &mut entries);
    entries
}

fn assert_git_error<T>(result: Result<T, GitError>) -> String {
    match result {
        Err(GitError::Git(message)) => {
            assert!(!message.trim().is_empty(), "Git errors must be actionable");
            message
        }
        Err(other) => panic!("expected Git error, got {other}"),
        Ok(_) => panic!("expected Git error, got success"),
    }
}
