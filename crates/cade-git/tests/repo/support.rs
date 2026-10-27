//! Shared owned-repository fixtures for integration and private-boundary probes.

use super::GitError;
use std::path::Path;
use std::process::Command;

pub(super) fn git(dir: &Path, args: &[&str]) -> Vec<u8> {
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
            "-c",
            "maintenance.auto=false",
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

pub(super) fn committed_repo() -> tempfile::TempDir {
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
pub(super) fn snapshot(root: &Path) -> std::collections::BTreeMap<std::path::PathBuf, (std::fs::FileType, Vec<u8>)> {
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

pub(super) fn assert_git_error<T>(result: Result<T, GitError>) -> String {
    match result {
        Err(GitError::Git(message)) => {
            assert!(!message.trim().is_empty(), "Git errors must be actionable");
            message
        }
        Err(other) => panic!("expected Git error, got {other}"),
        Ok(_) => panic!("expected Git error, got success"),
    }
}
