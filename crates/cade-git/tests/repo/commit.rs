//! Commit contract probes. Git CLI calls only build fixtures or independently
//! inspect results; every operation under test calls `ProjectRepo::commit`.
//! Run with global/system Git configuration disabled, as in the lane handoff.

use super::{ProjectRepo, assert_git_error, committed_repo, git, run_commit_message, snapshot};
use gix::objs::WriteTo;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[path = "commit/failures.rs"]
mod failures;
#[path = "commit/validation.rs"]
pub(super) mod validation;

isolated_tests! {
const REFERENCE: &str = "refs/heads/main";

pub(super) fn resolve(dir: &Path, revision: &str) -> gix::ObjectId {
    let output = git(dir, &["rev-parse", "--verify", revision]);
    gix::ObjectId::from_hex(String::from_utf8(output).unwrap().trim().as_bytes()).unwrap()
}

pub(super) fn sdk(dir: &Path) -> gix::Repository {
    gix::open_opts(dir, gix::open::Options::isolated()).unwrap()
}

// This immutable tree deliberately differs from both the index and worktree.
pub(super) fn request(dir: &Path, parents: &[gix::ObjectId]) -> gix::objs::Commit {
    assert_eq!(
        std::env::var("CADE_GIT_TEST_CHILD").ok().as_deref(),
        std::thread::current().name(),
        "commit fixtures must execute in their selected isolated child"
    );
    let repo = sdk(dir);
    let selected = repo.write_blob(b"selected document bytes\n").unwrap().detach();
    let original = repo.write_blob(b"original\n").unwrap().detach();
    let tree = repo
        .write_object(&gix::objs::Tree {
            entries: vec![
                gix::objs::tree::Entry {
                    mode: gix::objs::tree::EntryKind::Blob.into(),
                    filename: "selected.cade".into(),
                    oid: selected,
                },
                gix::objs::tree::Entry {
                    mode: gix::objs::tree::EntryKind::Blob.into(),
                    filename: "tracked.txt".into(),
                    oid: original,
                },
            ],
        })
        .unwrap()
        .detach();
    gix::objs::Commit {
        tree,
        parents: parents.iter().copied().collect(),
        author: gix::actor::Signature {
            name: "Supplied Author".into(),
            email: "author@example.invalid".into(),
            time: gix::date::Time {
                seconds: 946_684_800,
                offset: -25_200,
            },
        },
        committer: gix::actor::Signature {
            name: "Supplied Committer".into(),
            email: "committer@example.invalid".into(),
            time: gix::date::Time {
                seconds: 946_688_400,
                offset: 19_800,
            },
        },
        encoding: None,
        message: "Ordinary save\n\nUser-supplied body.\n".into(),
        extra_headers: Vec::new(),
    }
}

fn stored_commit(dir: &Path, id: gix::ObjectId) -> gix::objs::Commit {
    let bytes = git(dir, &["cat-file", "commit", &id.to_string()]);
    gix::objs::CommitRef::from_bytes(&bytes, gix::hash::Kind::Sha1)
        .unwrap()
        .to_owned()
        .unwrap()
}

fn assert_stored(dir: &Path, expected: &gix::objs::Commit, id: gix::ObjectId) {
    assert_eq!(resolve(dir, REFERENCE), id, "the returned object must be the actual branch tip");
    let mut expected_bytes = Vec::new();
    expected.write_to(&mut expected_bytes).unwrap();
    assert_eq!(
        git(dir, &["cat-file", "commit", &id.to_string()]),
        expected_bytes,
        "all supplied commit bytes must match SDK serialization"
    );
    assert_eq!(
        git(dir, &["ls-tree", "--name-only", &id.to_string()]),
        b"selected.cade\ntracked.txt\n",
        "unselected paths must not enter the tree"
    );
    assert_eq!(git(dir, &["show", &format!("{id}:selected.cade")]), b"selected document bytes\n");
    assert_eq!(git(dir, &["show", &format!("{id}:tracked.txt")]), b"original\n");
}

fn commit_success(repo: &ProjectRepo, dir: &Path, commit: &gix::objs::Commit) -> gix::ObjectId {
    let result = repo.commit(REFERENCE, commit);
    assert!(result.is_ok(), "valid explicit commit must succeed: {result:?}");
    let id = result.unwrap();
    assert_stored(dir, commit, id);
    id
}

// Failed ref transactions may leave unreachable immutable objects, not changes
// to index, worktree, refs, reflogs, HEAD, config or somebody else's lock.
fn without_objects(dir: &Path) -> BTreeMap<PathBuf, (fs::FileType, Vec<u8>)> {
    let mut state = snapshot(dir);
    state.retain(|path, _| !path.starts_with(".git/objects"));
    state
}

// A successful main update may add its reflog and the associated HEAD reflog,
// but may not alter HEAD attachment or any other reference.
fn protected_state(dir: &Path) -> BTreeMap<PathBuf, (fs::FileType, Vec<u8>)> {
    let mut state = without_objects(dir);
    state.retain(|path, _| {
        ![
            ".git/refs/heads/main",
            ".git/logs",
            ".git/logs/refs",
            ".git/logs/refs/heads",
            ".git/logs/refs/heads/main",
            ".git/logs/HEAD",
        ]
        .iter()
        .any(|allowed| path == Path::new(allowed))
    });
    state
}

pub(super) fn dirty_selection(dir: &Path) {
    fs::write(dir.join("tracked.txt"), b"unselected staged tracked bytes\n").unwrap();
    fs::write(dir.join("selected.cade"), b"unselected staged document bytes\n").unwrap();
    fs::write(dir.join("unselected.txt"), b"unselected new staged bytes\n").unwrap();
    git(dir, &["add", "--", "tracked.txt", "selected.cade", "unselected.txt"]);
    fs::write(dir.join("tracked.txt"), b"unselected dirty tracked bytes\n").unwrap();
    fs::write(dir.join("selected.cade"), b"unselected dirty document bytes\n").unwrap();
    fs::write(dir.join("untracked.txt"), b"unselected untracked bytes\n").unwrap();
}

pub(super) fn initial_repo() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q", "-b", "main"]);
    dir
}

#[test]
fn commit_selected_tree_records_parent_and_preserves_unselected_state() {
    let dir = committed_repo();
    let parent = resolve(dir.path(), REFERENCE);
    git(dir.path(), &["update-ref", "refs/heads/other", &parent.to_string()]);
    git(dir.path(), &["update-ref", "refs/tags/keep", &parent.to_string()]);
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let commit = request(dir.path(), &[parent]);
    // Mutate the index after both opening ProjectRepo and selecting the tree.
    dirty_selection(dir.path());
    let before = protected_state(dir.path());
    let index = dir.path().join(".git/index");
    let index_before = fs::metadata(&index).unwrap();

    commit_success(&repo, dir.path(), &commit);

    assert_eq!(protected_state(dir.path()), before);
    let index_after = fs::metadata(index).unwrap();
    assert_eq!(index_after.modified().unwrap(), index_before.modified().unwrap());
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        assert_eq!(
            index_after.ino(),
            index_before.ino(),
            "the index must not be replaced, even with identical bytes"
        );
    }
}

#[test]
fn commit_initial_selected_tree_excludes_unselected_staged_and_dirty_bytes() {
    let dir = initial_repo();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let commit = request(dir.path(), &[]);
    dirty_selection(dir.path());
    let before = protected_state(dir.path());

    commit_success(&repo, dir.path(), &commit);

    assert_eq!(protected_state(dir.path()), before);
    assert_eq!(git(dir.path(), &["rev-list", "--count", REFERENCE]), b"1\n");
}

#[test]
fn commit_initial_without_index_preserves_index_absence() {
    let dir = initial_repo();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let commit = request(dir.path(), &[]);
    assert!(!dir.path().join(".git/index").exists());
    let before = protected_state(dir.path());

    commit_success(&repo, dir.path(), &commit);

    assert!(!dir.path().join(".git/index").exists());
    assert!(!dir.path().join(".git/index.lock").exists());
    assert_eq!(protected_state(dir.path()), before);
}

#[test]
fn commit_applied_run_stores_existing_helper_trailers() {
    let dir = committed_repo();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let mut commit = request(dir.path(), &[resolve(dir.path(), REFERENCE)]);
    commit.message = run_commit_message("Apply selected document", "run-42", "fixture-model", 186_000).into();

    let id = commit_success(&repo, dir.path(), &commit);

    assert_eq!(
        git(dir.path(), &["show", "-s", "--format=%(trailers:only,unfold)", &id.to_string()]),
        b"CADE-Copilot-Run: run-42\nCADE-Model: fixture-model\nCADE-Tokens: 186000\n\n"
    );
}

#[test]
fn commit_ordinary_metadata_preserves_bytes_without_copilot_attribution() {
    let dir = committed_repo();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let mut commit = request(dir.path(), &[resolve(dir.path(), REFERENCE)]);
    commit.message = b"Save caf\xe9\n\nAn ordinary user's bytes.\n".to_vec().into();
    commit.encoding = Some("ISO-8859-1".into());
    commit.extra_headers.push(("x-fixture".into(), "first line\nsecond line".into()));

    let id = commit_success(&repo, dir.path(), &commit);
    let stored = stored_commit(dir.path(), id);

    assert!(!stored.message.windows(b"CADE-".len()).any(|bytes| bytes == b"CADE-"));
    assert_eq!(stored.author, commit.author);
    assert_eq!(stored.committer, commit.committer);
    assert_eq!(stored.encoding, commit.encoding);
    let mut expected_bytes = Vec::new();
    commit.write_to(&mut expected_bytes).unwrap();
    // The SDK decoder retains the terminating newline of multiline headers.
    let expected = gix::objs::CommitRef::from_bytes(&expected_bytes, gix::hash::Kind::Sha1)
        .unwrap()
        .to_owned()
        .unwrap();
    assert_eq!(stored.extra_headers, expected.extra_headers);
}

#[test]
fn commit_empty_message_preserves_empty_bytes() {
    let dir = committed_repo();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let mut commit = request(dir.path(), &[resolve(dir.path(), REFERENCE)]);
    commit.message = Vec::<u8>::new().into();

    let id = commit_success(&repo, dir.path(), &commit);

    assert!(stored_commit(dir.path(), id).message.is_empty());
}

#[test]
fn commit_supplied_committer_writes_reflog_without_configured_identity() {
    let dir = committed_repo();
    // The fixture CLI supplies its identity through -c, not persisted config.
    // Empty local values shadow any host identity; the supplied SDK identity wins.
    git(dir.path(), &["config", "user.name", ""]);
    git(dir.path(), &["config", "user.email", ""]);
    git(dir.path(), &["config", "user.useConfigOnly", "true"]);
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let commit = request(dir.path(), &[resolve(dir.path(), REFERENCE)]);
    let before = protected_state(dir.path());

    let id = commit_success(&repo, dir.path(), &commit);

    let log = fs::read_to_string(dir.path().join(".git/logs/refs/heads/main")).unwrap();
    let last = log.lines().last().unwrap();
    assert!(
        last.starts_with(&format!(
            "{} {id} Supplied Committer <committer@example.invalid> 946688400 +0530\t",
            commit.parents[0]
        )),
        "reflog must use the supplied committer and timestamp: {last}"
    );
    assert_eq!(protected_state(dir.path()), before);
}

#[test]
fn commit_detached_head_updates_only_requested_branch() {
    let dir = committed_repo();
    let parent = resolve(dir.path(), REFERENCE);
    git(dir.path(), &["checkout", "-q", "--detach"]);
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let commit = request(dir.path(), &[parent]);
    let before = protected_state(dir.path());
    let head_log = fs::read(dir.path().join(".git/logs/HEAD")).unwrap();

    commit_success(&repo, dir.path(), &commit);

    assert_eq!(resolve(dir.path(), "HEAD"), parent);
    assert_eq!(repo.branch().unwrap(), None);
    assert_eq!(protected_state(dir.path()), before);
    assert_eq!(fs::read(dir.path().join(".git/logs/HEAD")).unwrap(), head_log);
}

#[test]
fn commit_head_attached_elsewhere_preserves_head_and_other_branch() {
    let dir = committed_repo();
    let parent = resolve(dir.path(), REFERENCE);
    git(dir.path(), &["checkout", "-q", "-b", "other"]);
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let commit = request(dir.path(), &[parent]);
    let before = protected_state(dir.path());
    let head_log = fs::read(dir.path().join(".git/logs/HEAD")).unwrap();

    commit_success(&repo, dir.path(), &commit);

    assert_eq!(resolve(dir.path(), "HEAD"), parent);
    assert_eq!(resolve(dir.path(), "refs/heads/other"), parent);
    assert_eq!(repo.branch().unwrap().as_deref(), Some("other"));
    assert_eq!(protected_state(dir.path()), before);
    assert_eq!(fs::read(dir.path().join(".git/logs/HEAD")).unwrap(), head_log);
}

#[test]
fn commit_merge_parents_preserves_supplied_order() {
    let dir = committed_repo();
    let parent = resolve(dir.path(), REFERENCE);
    let mut second = request(dir.path(), &[]);
    second.message = "Independent second parent\n".into();
    let second_id = sdk(dir.path()).write_object(&second).unwrap().detach();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let commit = request(dir.path(), &[parent, second_id]);

    let id = commit_success(&repo, dir.path(), &commit);

    assert_eq!(
        git(dir.path(), &["show", "-s", "--format=%P", &id.to_string()]),
        format!("{parent} {second_id}\n").into_bytes()
    );
}
}
