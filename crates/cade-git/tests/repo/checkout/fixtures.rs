//! One shared checkout fixture, including the early-write preservation canary.

use super::{committed_repo, git};
use std::fs;

pub(super) const REFERENCE: &str = "refs/heads/topic";

pub(super) fn actor() -> gix::actor::SignatureRef<'static> {
    gix::actor::SignatureRef::from_bytes(b"Checkout Actor <checkout@example.invalid> 946688400 +0530").unwrap()
}

pub(super) fn fixture() -> tempfile::TempDir {
    let dir = committed_repo();
    for (name, bytes) in [
        (".gitignore", "z-ignored\nkeep-ignored\n"),
        ("keep-stage", "base staged candidate\n"),
        ("keep-work", "base work candidate\n"),
        ("removed.txt", "removed on topic\n"),
    ] {
        fs::write(dir.path().join(name), bytes).unwrap();
    }
    git(dir.path(), &["add", "-A"]);
    git(dir.path(), &["commit", "-q", "-m", "Checkout baseline"]);
    git(dir.path(), &["checkout", "-q", "-b", "topic"]);
    fs::write(dir.path().join("tracked.txt"), b"topic tracked bytes\n").unwrap();
    fs::remove_file(dir.path().join("removed.txt")).unwrap();
    fs::create_dir(dir.path().join("targetdir")).unwrap();
    for (name, bytes) in [
        ("a-early", "first changed path\n"),
        ("targetdir/child", "topic child\n"),
        ("z-new", "topic new file\n"),
        ("z-ignored", "topic tracked ignored name\n"),
    ] {
        fs::write(dir.path().join(name), bytes).unwrap();
    }
    git(dir.path(), &["add", "-f", "-A"]);
    git(dir.path(), &["commit", "-q", "-m", "Checkout target"]);
    git(dir.path(), &["checkout", "-q", "main"]);
    dir
}
