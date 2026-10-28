//! Filesystem integration probes compiled only into the library test target to
//! reach its private observer. Every behavior invokes ProjectRepo::checkout.

use super::observer::{self, Phase};
use crate::{GitError, ProjectRepo};
use std::{cell::RefCell, fs, path::Path, rc::Rc};

#[path = "../repo/checkout/fixtures.rs"]
mod fixtures;
#[macro_use]
#[path = "../repo/support.rs"]
pub(crate) mod support;

use fixtures::{REFERENCE, actor, fixture};
use support::{assert_git_error, committed_repo, git, run_isolated, snapshot};

mod failures;
mod guards;
mod isolation;

fn dirty_selection(dir: &Path) {
    fs::write(dir.join("keep-stage"), b"boundary staged bytes\n").unwrap();
    git(dir, &["add", "--", "keep-stage"]);
    fs::write(dir.join("keep-stage"), b"boundary unstaged bytes\n").unwrap();
    fs::write(dir.join("keep-work"), b"boundary tracked edit\n").unwrap();
    fs::write(dir.join("keep-untracked"), b"boundary untracked bytes\n").unwrap();
    fs::write(dir.join("keep-ignored"), b"boundary ignored bytes\n").unwrap();
}

fn assert_selection(dir: &Path) {
    assert_eq!(git(dir, &["show", ":keep-stage"]), b"boundary staged bytes\n");
    assert_worktree_selection(dir);
}

fn assert_worktree_selection(dir: &Path) {
    for (name, bytes) in [
        ("keep-stage", b"boundary unstaged bytes\n".as_slice()),
        ("keep-work", b"boundary tracked edit\n"),
        ("keep-untracked", b"boundary untracked bytes\n"),
        ("keep-ignored", b"boundary ignored bytes\n"),
    ] {
        assert_eq!(fs::read(dir.join(name)).unwrap(), bytes);
    }
}

fn assert_target(dir: &Path) {
    assert_eq!(git(dir, &["symbolic-ref", "HEAD"]), b"refs/heads/topic\n");
    assert_eq!(git(dir, &["rev-parse", "HEAD"]), git(dir, &["rev-parse", REFERENCE]));
    for name in ["tracked.txt", "a-early", "targetdir/child", "z-new", "z-ignored"] {
        let expected = git(dir, &["show", &format!("{REFERENCE}:{name}")]);
        assert_eq!(fs::read(dir.join(name)).unwrap(), expected);
        assert_eq!(git(dir, &["show", &format!(":{name}")]), expected);
    }
    assert!(!dir.join("removed.txt").exists());
    assert!(git(dir, &["ls-files", "--", "removed.txt"]).is_empty());
}

fn assert_unlocked(dir: &Path) {
    for name in ["index", "HEAD", "refs/heads/main", REFERENCE] {
        let path = dir.join(".git").join(name);
        let lock = gix::lock::Marker::acquire_to_hold_resource(path, gix::lock::acquire::Fail::Immediately, None, 0)
            .expect("checkout must release every owned guard before returning");
        drop(lock);
    }
}
