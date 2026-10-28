//! Public clone/fetch denial and destination-preservation integration tests.

use cade_git::{GitError, ProjectRepo};
use cade_github::Host;
use cade_policy::{Policy, Switch};
use octocrab::secrecy::SecretString;
use std::{fs, path::Path, sync::atomic::AtomicBool};
use url::Url;

#[macro_use]
#[path = "repo/support.rs"]
mod support;
use support::{assert_git_error, committed_repo, run_isolated, snapshot};

fn clone(destination: &Path, policy: Option<&Policy>, token: &str) -> Result<ProjectRepo, GitError> {
    ProjectRepo::clone(
        destination,
        &Url::parse("https://forge.example.invalid/synthetic/project.git").unwrap(),
        &Host::Server(Url::parse("https://forge.example.invalid/").unwrap()),
        &SecretString::from(token.to_owned()),
        policy,
        gix::progress::Discard,
        &AtomicBool::new(false),
    )
}

isolated_tests! {
#[test]
fn clone_missing_policy_preserves_absent_destination() {
    let dir = tempfile::tempdir().unwrap();
    let destination = dir.path().join("not-created");
    assert_git_error(clone(&destination, None, "synthetic-token"));
    assert_eq!(snapshot(dir.path()), Default::default());
}

#[test]
fn clone_network_denied_preserves_absent_destination() {
    let dir = tempfile::tempdir().unwrap();
    let mut policy = Policy::default();
    policy.tools.network = Switch::Off;
    assert_git_error(clone(&dir.path().join("not-created"), Some(&policy), "synthetic-token"));
    assert_eq!(snapshot(dir.path()), Default::default());
}

#[test]
fn clone_empty_token_preserves_absent_destination() {
    let dir = tempfile::tempdir().unwrap();
    assert_git_error(clone(&dir.path().join("not-created"), Some(&Policy::default()), ""));
    assert_eq!(snapshot(dir.path()), Default::default());
}

#[test]
fn clone_unselected_host_preserves_absent_destination() {
    let dir = tempfile::tempdir().unwrap();
    assert_git_error(ProjectRepo::clone(
        &dir.path().join("not-created"),
        &Url::parse("https://other.example.invalid/synthetic/project.git").unwrap(),
        &Host::Server(Url::parse("https://forge.example.invalid/").unwrap()),
        &SecretString::from("synthetic-token".to_owned()),
        Some(&Policy::default()),
        gix::progress::Discard,
        &AtomicBool::new(false),
    ));
    assert_eq!(snapshot(dir.path()), Default::default());
}

#[test]
fn clone_occupied_directory_preserves_contents() {
    let dir = committed_repo();
    fs::write(dir.path().join("untracked"), b"owned content\n").unwrap();
    let before = snapshot(dir.path());
    assert_git_error(clone(dir.path(), Some(&Policy::default()), "synthetic-token"));
    assert_eq!(snapshot(dir.path()), before);
}

#[test]
fn clone_empty_directory_preserves_directory() {
    let dir = tempfile::tempdir().unwrap();
    let destination = dir.path().join("already-owned");
    fs::create_dir(&destination).unwrap();
    let before = snapshot(dir.path());
    assert_git_error(clone(&destination, Some(&Policy::default()), "synthetic-token"));
    assert_eq!(snapshot(dir.path()), before);
}

#[cfg(any(unix, windows))]
#[test]
fn clone_dangling_symlink_preserves_link() {
    let dir = tempfile::tempdir().unwrap();
    let destination = dir.path().join("already-owned");
    #[cfg(unix)]
    std::os::unix::fs::symlink("missing-target", &destination).unwrap();
    #[cfg(windows)]
    std::os::windows::fs::symlink_dir("missing-target", &destination).expect("Windows symlink privilege or Developer Mode");
    let before = snapshot(dir.path());
    assert_git_error(clone(&destination, Some(&Policy::default()), "synthetic-token"));
    assert_eq!(snapshot(dir.path()), before);
}

#[test]
fn fetch_missing_policy_preserves_repository() {
    let dir = committed_repo();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let before = snapshot(dir.path());
    assert_git_error(repo.fetch(
        &Url::parse("https://forge.example.invalid/synthetic/project.git").unwrap(),
        &Host::Server(Url::parse("https://forge.example.invalid/").unwrap()),
        &SecretString::from("synthetic-token".to_owned()),
        None,
        gix::progress::Discard,
        &AtomicBool::new(false),
    ));
    assert_eq!(snapshot(dir.path()), before);
}

#[test]
fn fetch_network_denied_preserves_repository() {
    let dir = committed_repo();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let mut policy = Policy::default();
    policy.tools.network = Switch::Off;
    let before = snapshot(dir.path());
    assert_git_error(repo.fetch(
        &Url::parse("https://forge.example.invalid/synthetic/project.git").unwrap(),
        &Host::Server(Url::parse("https://forge.example.invalid/").unwrap()),
        &SecretString::from("synthetic-token".to_owned()),
        Some(&policy),
        gix::progress::Discard,
        &AtomicBool::new(false),
    ));
    assert_eq!(snapshot(dir.path()), before);
}

#[test]
fn fetch_empty_token_preserves_repository() {
    let dir = committed_repo();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let before = snapshot(dir.path());
    assert_git_error(repo.fetch(
        &Url::parse("https://forge.example.invalid/synthetic/project.git").unwrap(),
        &Host::Server(Url::parse("https://forge.example.invalid/").unwrap()),
        &SecretString::from(String::new()),
        Some(&Policy::default()),
        gix::progress::Discard,
        &AtomicBool::new(false),
    ));
    assert_eq!(snapshot(dir.path()), before);
}
}
