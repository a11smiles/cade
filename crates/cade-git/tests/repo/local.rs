//! Strict initialization and opening boundaries; basic controls remain in repo.rs.

use super::{GitError, ProjectRepo, assert_git_error, committed_repo, git, snapshot};
use std::fs;

#[test]
fn init_unrelated_files_preserves_contents() {
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir(dir.path().join("notes")).unwrap();
    fs::write(dir.path().join("notes/keep.txt"), b"unrelated\n").unwrap();
    let before = snapshot(dir.path());

    assert!(ProjectRepo::init(dir.path()).is_ok());

    let mut after = snapshot(dir.path());
    after.retain(|path, _| !path.starts_with(".git"));
    assert_eq!(after, before);
}

#[test]
fn init_occupied_git_file_rejects_without_mutation() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join(".git"), b"unrelated metadata\n").unwrap();
    let before = snapshot(dir.path());

    assert_git_error(ProjectRepo::init(dir.path()));
    assert_eq!(snapshot(dir.path()), before);
}

#[test]
fn init_occupied_git_directory_rejects_without_mutation() {
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir(dir.path().join(".git")).unwrap();
    fs::write(dir.path().join(".git/keep"), b"not a repository\n").unwrap();
    let before = snapshot(dir.path());

    assert_git_error(ProjectRepo::init(dir.path()));
    assert_eq!(snapshot(dir.path()), before);
}

#[cfg(unix)]
#[test]
fn init_dangling_git_symlink_rejects_without_mutation() {
    let dir = tempfile::tempdir().unwrap();
    std::os::unix::fs::symlink("missing-metadata", dir.path().join(".git")).unwrap();
    let before = snapshot(dir.path());

    assert_git_error(ProjectRepo::init(dir.path()));
    assert_eq!(snapshot(dir.path()), before);
    assert!(!dir.path().join("missing-metadata").exists());
}

#[test]
fn init_bare_repository_rejects_without_mutation() {
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q", "--bare", "-b", "main"]);
    let before = snapshot(dir.path());

    assert_git_error(ProjectRepo::init(dir.path()));
    assert_eq!(snapshot(dir.path()), before);
}

#[test]
fn open_existing_repository_preserves_metadata_and_worktree() {
    let dir = committed_repo();
    git(dir.path(), &["config", "cade.fixture", "keep"]);
    let before = snapshot(dir.path());

    assert!(ProjectRepo::open(dir.path()).is_ok());
    assert_eq!(snapshot(dir.path()), before);
}

#[test]
fn open_corrupt_config_returns_git_error_and_recovers() {
    let dir = committed_repo();
    let path = dir.path().join(".git/config");
    let original = fs::read(&path).unwrap();
    fs::write(&path, b"[unterminated\n").unwrap();
    let before = snapshot(dir.path());

    let result = ProjectRepo::open(dir.path());
    assert_eq!(snapshot(dir.path()), before);
    assert_git_error(result);

    fs::write(path, original).unwrap();
    assert!(ProjectRepo::open(dir.path()).is_ok());
}

#[test]
fn open_corrupt_head_returns_git_error_and_recovers() {
    let dir = committed_repo();
    let path = dir.path().join(".git/HEAD");
    let original = fs::read(&path).unwrap();
    fs::write(&path, b"not a ref or object id\n").unwrap();
    let before = snapshot(dir.path());

    let result = ProjectRepo::open(dir.path());
    assert_eq!(snapshot(dir.path()), before);
    assert_git_error(result);

    fs::write(path, original).unwrap();
    assert!(ProjectRepo::open(dir.path()).is_ok());
}

#[test]
fn open_missing_repository_created_later_recovers() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("created-later");
    assert!(matches!(ProjectRepo::open(&path), Err(GitError::NotARepo(p)) if p == path));
    fs::create_dir(&path).unwrap();
    git(&path, &["init", "-q", "-b", "main"]);

    assert!(ProjectRepo::open(&path).is_ok());
}

#[cfg(unix)]
#[test]
fn open_unreadable_config_returns_git_error_and_recovers() {
    use std::os::unix::fs::PermissionsExt;

    let dir = committed_repo();
    let path = dir.path().join(".git/config");
    let original = fs::metadata(&path).unwrap().permissions();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o0)).unwrap();
    let denied = fs::read(&path);
    let result = ProjectRepo::open(dir.path());
    fs::set_permissions(&path, original).unwrap();

    // A privileged runner must report this fixture as unexecuted, not skip it
    // or count its successful read as evidence of denial.
    assert_eq!(denied.unwrap_err().kind(), std::io::ErrorKind::PermissionDenied);
    assert_git_error(result);
    assert!(ProjectRepo::open(dir.path()).is_ok());
}
