//! Shared synthetic object fixtures, independent of the operations under test.

use super::commit::{request, sdk};
use super::git;
use std::path::{Path, PathBuf};

pub(super) fn write_commit(dir: &Path, parents: &[gix::ObjectId], message: &str) -> gix::ObjectId {
    let mut commit = request(dir, parents);
    commit.message = message.into();
    sdk(dir).write_object(&commit).unwrap().detach()
}

pub(super) fn loose_object(dir: &Path, id: gix::ObjectId) -> PathBuf {
    let hex = id.to_string();
    sdk(dir).git_dir().join("objects").join(&hex[..2]).join(&hex[2..])
}

pub(super) fn malformed_commit(dir: &Path) -> gix::ObjectId {
    // A valid loose-object envelope with invalid commit payload exercises
    // decoding, not merely object lookup, kind checks or decompression.
    let path = dir.join("malformed-commit-fixture");
    std::fs::write(&path, b"not a decodable commit\n").unwrap();
    let output = git(
        dir,
        &["hash-object", "--literally", "-t", "commit", "-w", "malformed-commit-fixture"],
    );
    std::fs::remove_file(path).unwrap();
    gix::ObjectId::from_hex(String::from_utf8(output).unwrap().trim().as_bytes()).unwrap()
}
