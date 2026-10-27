//! Production checkout contract probes, using only owned temporary repositories.
//! Git builds/inspects fixtures; the operation under test is always ProjectRepo.

use super::commit::{resolve, sdk};
use super::objects::{loose_object, malformed_commit};
use super::{ProjectRepo, assert_git_error, committed_repo, git, snapshot};
use gix::bstr::ByteSlice;
use std::fs;
use std::path::Path;

#[path = "checkout/content.rs"]
mod content;
#[path = "checkout/denials.rs"]
mod denials;
#[path = "checkout/fixtures.rs"]
mod fixtures;
#[path = "checkout/state.rs"]
mod state;
#[path = "checkout/validation.rs"]
mod validation;
#[path = "checkout/worktrees.rs"]
mod worktrees;

use fixtures::{REFERENCE, actor, fixture};

fn switch(repo: &ProjectRepo, dir: &Path) {
    let target = resolve(dir, REFERENCE);
    let result = repo.checkout(REFERENCE, actor());
    assert!(result.is_ok(), "valid checkout must succeed: {result:?}");
    assert_eq!(git(dir, &["symbolic-ref", "HEAD"]), b"refs/heads/topic\n");
    assert_eq!(resolve(dir, "HEAD"), target);
    assert_eq!(resolve(dir, REFERENCE), target);
    assert_eq!(fs::read(dir.join("tracked.txt")).unwrap(), b"topic tracked bytes\n");
    assert_eq!(fs::read(dir.join("a-early")).unwrap(), b"first changed path\n");
    assert_eq!(fs::read(dir.join("targetdir/child")).unwrap(), b"topic child\n");
    assert_eq!(fs::read(dir.join("z-new")).unwrap(), b"topic new file\n");
    assert_eq!(fs::read(dir.join("z-ignored")).unwrap(), b"topic tracked ignored name\n");
    assert!(!dir.join("removed.txt").exists());
    for name in ["tracked.txt", "a-early", "targetdir/child", "z-new", "z-ignored"] {
        assert_eq!(
            git(dir, &["show", &format!(":{name}")]),
            git(dir, &["show", &format!("{REFERENCE}:{name}")]),
            "published index content must match the selected branch for {name}"
        );
    }
    assert!(git(dir, &["ls-files", "--", "removed.txt"]).is_empty());
}

fn assert_category(message: &str, alternatives: &[&str]) {
    let lower = message.to_ascii_lowercase();
    assert!(
        alternatives.iter().any(|category| lower.contains(category)),
        "denial must identify one of {alternatives:?}, not merely return a generic error"
    );
}

fn deny(repo: &ProjectRepo, dir: &Path, reference: &str, committer: gix::actor::SignatureRef<'_>, categories: &[&str]) {
    let before = snapshot(dir);
    // The existing snapshot preserves native names, symlinks, empty directories,
    // objects, index bytes, refs/logs, config and foreign locks. Also retain modes.
    let permissions: Vec<_> = before
        .keys()
        .map(|path| (path.clone(), fs::symlink_metadata(dir.join(path)).unwrap().permissions()))
        .collect();
    let result = repo.checkout(reference, committer);
    assert_eq!(snapshot(dir), before, "known denial must precede every content/metadata write");
    for (path, expected) in permissions {
        assert_eq!(fs::symlink_metadata(dir.join(path)).unwrap().permissions(), expected);
    }
    let error = result.as_ref().expect_err("unsafe checkout must be denied");
    for diagnostic in [error.to_string(), format!("{error:?}"), format!("{error:#?}")] {
        assert!(
            !diagnostic.contains("checkout-raw-canary"),
            "diagnostic must not disclose raw actor/object bytes"
        );
    }
    let message = assert_git_error(result);
    assert_category(&message, categories);
}

fn deny_default(repo: &ProjectRepo, dir: &Path, categories: &[&str]) {
    deny(repo, dir, REFERENCE, actor(), categories);
}

fn commit_target_changes(dir: &Path) {
    git(dir, &["add", "-f", "-A"]);
    git(dir, &["commit", "-q", "-m", "Target variation"]);
    git(dir, &["checkout", "-q", "main"]);
}

fn replace_target(dir: &Path, id: gix::ObjectId) {
    // Deliberately invalid ref targets cannot be installed by update-ref.
    // This writes a loose ref fixture, never an index/object codec.
    fs::write(dir.join(".git").join(REFERENCE), format!("{id}\n")).unwrap();
}
