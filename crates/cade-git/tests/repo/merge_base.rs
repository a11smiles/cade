//! Explicit-OID ancestry and read-only failure boundaries in real repositories.

use super::commit::validation::absent_object;
use super::commit::{dirty_selection, initial_repo, resolve, sdk};
use super::objects::{loose_object, malformed_commit, write_commit};
use super::{ProjectRepo, assert_git_error, committed_repo, git, run_isolated, snapshot};
use std::fs;
use std::path::Path;

isolated_tests! {
fn merge_base_success(repo: &ProjectRepo, dir: &Path, one: gix::ObjectId, two: gix::ObjectId, expected: Option<gix::ObjectId>) {
    let before = snapshot(dir);

    let result = repo.merge_base(one, two);

    assert!(
        result.is_ok(),
        "available commit inputs must yield an explicit ancestry result: {result:?}"
    );
    assert_eq!(result.unwrap(), expected);
    assert_eq!(snapshot(dir), before, "ancestry must not change any repository or worktree bytes");
}

fn reject_inputs(repo: &ProjectRepo, dir: &Path, invalid: gix::ObjectId, valid: gix::ObjectId) {
    // Both positions and the equality fast path must validate object decoding.
    for (one, two) in [(invalid, valid), (valid, invalid), (invalid, invalid)] {
        let before = snapshot(dir);

        assert_git_error(repo.merge_base(one, two));

        assert_eq!(snapshot(dir), before);
    }
}

fn invalid_then_corrected(dir: &Path, invalid: gix::ObjectId) {
    let valid = resolve(dir, "HEAD");
    let repo = ProjectRepo::open(dir).unwrap();
    reject_inputs(&repo, dir, invalid, valid);
    merge_base_success(&repo, dir, valid, valid, Some(valid));
}

#[test]
fn merge_base_diverging_histories_returns_unique_ancestor_without_mutating_dirty_state() {
    let dir = committed_repo();
    let root = resolve(dir.path(), "HEAD");
    let ancestor = write_commit(dir.path(), &[root], "Best common ancestor, not the root");
    let one = write_commit(dir.path(), &[ancestor], "First side");
    let two = write_commit(dir.path(), &[ancestor], "Second side");
    dirty_selection(dir.path());
    git(dir.path(), &["update-ref", "refs/heads/other", &two.to_string()]);
    fs::write(dir.path().join(".git/index.lock"), b"foreign index lock\n").unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();

    merge_base_success(&repo, dir.path(), one, two, Some(ancestor));
    merge_base_success(&repo, dir.path(), two, one, Some(ancestor));
}

#[test]
fn merge_base_unrelated_histories_returns_none_without_mutation() {
    let dir = committed_repo();
    let one = resolve(dir.path(), "HEAD");
    let two = write_commit(dir.path(), &[], "Independent root");
    let repo = ProjectRepo::open(dir.path()).unwrap();

    merge_base_success(&repo, dir.path(), one, two, None);
    merge_base_success(&repo, dir.path(), two, one, None);
}

#[test]
fn merge_base_identical_available_commit_returns_that_commit_without_mutation() {
    let dir = committed_repo();
    let id = resolve(dir.path(), "HEAD");
    let repo = ProjectRepo::open(dir.path()).unwrap();

    merge_base_success(&repo, dir.path(), id, id, Some(id));
}

#[test]
fn merge_base_ancestor_input_returns_ancestor_not_descendant() {
    let dir = committed_repo();
    let ancestor = resolve(dir.path(), "HEAD");
    let descendant = write_commit(dir.path(), &[ancestor], "Descendant");
    let repo = ProjectRepo::open(dir.path()).unwrap();

    merge_base_success(&repo, dir.path(), ancestor, descendant, Some(ancestor));
    merge_base_success(&repo, dir.path(), descendant, ancestor, Some(ancestor));
}

#[test]
fn merge_base_multiple_best_ancestors_returns_one_sdk_selected_base() {
    let dir = committed_repo();
    let root = resolve(dir.path(), "HEAD");
    let left = write_commit(dir.path(), &[root], "Left base");
    let right = write_commit(dir.path(), &[root], "Right base");
    let one = write_commit(dir.path(), &[left, right], "First crisscross merge");
    let two = write_commit(dir.path(), &[right, left], "Second crisscross merge");
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let sdk = sdk(dir.path());
    let all: std::collections::BTreeSet<_> = sdk
        .merge_bases_many(one, &[two])
        .unwrap()
        .into_iter()
        .map(|id| id.detach())
        .collect();
    assert_eq!(all, [left, right].into_iter().collect(), "fixture must really have two best bases");

    for (one, two) in [(one, two), (two, one)] {
        let expected = sdk.merge_base(one, two).unwrap().map(|id| id.detach());
        assert!(expected.is_some_and(|id| all.contains(&id)));
        merge_base_success(&repo, dir.path(), one, two, expected);
    }
}

#[test]
fn merge_base_missing_inputs_including_equal_oid_deny_then_available_input_succeeds() {
    let dir = committed_repo();
    invalid_then_corrected(dir.path(), absent_object(dir.path()));
}

#[test]
fn merge_base_blob_inputs_including_equal_oid_deny_then_commit_input_succeeds() {
    let dir = committed_repo();
    let blob = sdk(dir.path()).write_blob(b"not a commit").unwrap().detach();
    invalid_then_corrected(dir.path(), blob);
}

#[test]
fn merge_base_tree_inputs_including_equal_oid_deny_then_commit_input_succeeds() {
    let dir = committed_repo();
    invalid_then_corrected(dir.path(), resolve(dir.path(), "HEAD^{tree}"));
}

#[test]
fn merge_base_annotated_tag_inputs_including_equal_oid_deny_without_peeling() {
    let dir = committed_repo();
    git(dir.path(), &["tag", "-a", "fixture", "-m", "Annotated fixture"]);
    invalid_then_corrected(dir.path(), resolve(dir.path(), "refs/tags/fixture"));
}

#[test]
fn merge_base_malformed_commit_inputs_including_equal_oid_deny_then_valid_commit_succeeds() {
    let dir = committed_repo();
    let invalid = malformed_commit(dir.path());
    invalid_then_corrected(dir.path(), invalid);
}

#[test]
fn merge_base_corrupt_object_inputs_including_equal_oid_deny_then_repaired_object_succeeds() {
    let dir = committed_repo();
    let valid = resolve(dir.path(), "HEAD");
    let invalid = write_commit(dir.path(), &[], "Corruptible input");
    let path = loose_object(dir.path(), invalid);
    let bytes = fs::read(&path).unwrap();
    fs::remove_file(&path).unwrap();
    fs::write(&path, b"not compressed Git object bytes").unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();

    reject_inputs(&repo, dir.path(), invalid, valid);

    fs::write(path, bytes).unwrap();
    merge_base_success(&repo, dir.path(), invalid, invalid, Some(invalid));
}

#[test]
fn merge_base_missing_ancestor_returns_error_not_none_then_restored_graph_succeeds() {
    let dir = committed_repo();
    let ancestor = write_commit(dir.path(), &[], "Removable common ancestor");
    let one = write_commit(dir.path(), &[ancestor], "First side");
    let two = write_commit(dir.path(), &[ancestor], "Second side");
    let path = loose_object(dir.path(), ancestor);
    let bytes = fs::read(&path).unwrap();
    fs::remove_file(&path).unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let before = snapshot(dir.path());

    assert_git_error(repo.merge_base(one, two));
    assert_git_error(repo.merge_base(two, one));

    assert_eq!(snapshot(dir.path()), before);
    fs::write(path, bytes).unwrap();
    merge_base_success(&repo, dir.path(), one, two, Some(ancestor));
}

#[test]
fn merge_base_corrupt_ancestor_returns_error_not_none_then_repaired_graph_succeeds() {
    let dir = committed_repo();
    let ancestor = write_commit(dir.path(), &[], "Corruptible common ancestor");
    let one = write_commit(dir.path(), &[ancestor], "First side");
    let two = write_commit(dir.path(), &[ancestor], "Second side");
    let path = loose_object(dir.path(), ancestor);
    let bytes = fs::read(&path).unwrap();
    fs::remove_file(&path).unwrap();
    fs::write(&path, b"corrupt common ancestor storage").unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let before = snapshot(dir.path());

    assert_git_error(repo.merge_base(one, two));
    assert_git_error(repo.merge_base(two, one));

    assert_eq!(snapshot(dir.path()), before);
    fs::write(path, bytes).unwrap();
    merge_base_success(&repo, dir.path(), one, two, Some(ancestor));
}

#[test]
fn merge_base_malformed_ancestor_returns_error_instead_of_fabricated_absence() {
    let dir = committed_repo();
    let ancestor = malformed_commit(dir.path());
    let one = write_commit(dir.path(), &[ancestor], "First side");
    let two = write_commit(dir.path(), &[ancestor], "Second side");
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let before = snapshot(dir.path());

    assert_git_error(repo.merge_base(one, two));
    assert_git_error(repo.merge_base(two, one));

    assert_eq!(snapshot(dir.path()), before);
    // A fail-closed stub may already pass this denial-only control. That is not
    // evidence that graph validation has been implemented.
}

#[test]
fn merge_base_bare_repository_returns_available_ancestor_without_worktree() {
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q", "--bare", "-b", "main"]);
    let ancestor = write_commit(dir.path(), &[], "Bare root");
    let descendant = write_commit(dir.path(), &[ancestor], "Bare child");
    let repo = ProjectRepo::open(dir.path()).unwrap();

    merge_base_success(&repo, dir.path(), ancestor, descendant, Some(ancestor));

    assert!(repo.workdir().is_none());
}

#[test]
fn merge_base_unborn_head_returns_available_ancestor_without_creating_index_or_refs() {
    let dir = initial_repo();
    let ancestor = write_commit(dir.path(), &[], "Unreferenced root");
    let descendant = write_commit(dir.path(), &[ancestor], "Unreferenced child");
    let repo = ProjectRepo::open(dir.path()).unwrap();

    merge_base_success(&repo, dir.path(), ancestor, descendant, Some(ancestor));

    assert!(!repo.has_commits());
    assert!(!dir.path().join(".git/index").exists());
}

#[test]
fn merge_base_detached_head_returns_explicit_ancestor_without_reattaching() {
    let dir = committed_repo();
    let ancestor = resolve(dir.path(), "HEAD");
    let descendant = write_commit(dir.path(), &[ancestor], "Detached child");
    git(dir.path(), &["checkout", "-q", "--detach"]);
    let repo = ProjectRepo::open(dir.path()).unwrap();

    merge_base_success(&repo, dir.path(), ancestor, descendant, Some(ancestor));

    assert_eq!(repo.branch().unwrap(), None);
}

#[test]
fn merge_base_space_and_unicode_repository_path_preserves_native_content() {
    let parent = tempfile::tempdir().unwrap();
    let fixture = committed_repo();
    let path = parent.path().join("project space 日本語");
    fs::rename(fixture.path(), &path).unwrap();
    fs::write(path.join("document café.cade"), b"preserved native path content\n").unwrap();
    let ancestor = resolve(&path, "HEAD");
    let descendant = write_commit(&path, &[ancestor], "Native path child");
    let repo = ProjectRepo::open(&path).unwrap();

    merge_base_success(&repo, &path, ancestor, descendant, Some(ancestor));
}

// Match status's Linux-only raw-byte fixture: macOS rejects this directory
// rename with EILSEQ. Spaces/Unicode coverage above remains platform-independent.
#[cfg(target_os = "linux")]
#[test]
fn merge_base_non_utf8_repository_path_preserves_actual_native_bytes() {
    use std::os::unix::ffi::OsStringExt;

    let parent = tempfile::tempdir().unwrap();
    let fixture = committed_repo();
    let path = parent.path().join(std::ffi::OsString::from_vec(b"project-\xff".to_vec()));
    fs::rename(fixture.path(), &path).unwrap();
    fs::write(
        path.join(std::ffi::OsString::from_vec(b"document-\xfe".to_vec())),
        b"native bytes\n",
    )
    .unwrap();
    let ancestor = resolve(&path, "HEAD");
    let descendant = write_commit(&path, &[ancestor], "Native byte path child");
    let repo = ProjectRepo::open(&path).unwrap();

    merge_base_success(&repo, &path, ancestor, descendant, Some(ancestor));
}
}
