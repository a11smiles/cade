//! Invalid inputs preserve repository state, then an explicitly corrected
//! request succeeds on the same ProjectRepo. No automatic retry is exercised.

use super::*;

isolated_tests! {
fn invalid_then_corrected(dir: &Path, reference: &str, invalid: &gix::objs::Commit, valid: &gix::objs::Commit) {
    let repo = ProjectRepo::open(dir).unwrap();
    let before = without_objects(dir);

    assert_git_error(repo.commit(reference, invalid));

    assert_eq!(without_objects(dir), before);
    commit_success(&repo, dir, valid);
}

fn absent_object(dir: &Path) -> gix::ObjectId {
    let id = gix::ObjectId::from_hex(b"1111111111111111111111111111111111111111").unwrap();
    assert!(!sdk(dir).has_object(id), "missing-object fixture must really be absent");
    id
}

#[test]
fn commit_missing_tree_denies_then_corrected_request_succeeds() {
    let dir = committed_repo();
    let valid = request(dir.path(), &[resolve(dir.path(), REFERENCE)]);
    let mut invalid = valid.clone();
    invalid.tree = absent_object(dir.path());
    invalid_then_corrected(dir.path(), REFERENCE, &invalid, &valid);
}

#[test]
fn commit_blob_as_tree_denies_then_corrected_request_succeeds() {
    let dir = committed_repo();
    let valid = request(dir.path(), &[resolve(dir.path(), REFERENCE)]);
    let mut invalid = valid.clone();
    invalid.tree = sdk(dir.path()).write_blob(b"not a tree").unwrap().detach();
    invalid_then_corrected(dir.path(), REFERENCE, &invalid, &valid);
}

#[test]
fn commit_missing_first_parent_denies_then_corrected_request_succeeds() {
    let dir = committed_repo();
    let valid = request(dir.path(), &[resolve(dir.path(), REFERENCE)]);
    let mut invalid = valid.clone();
    invalid.parents[0] = absent_object(dir.path());
    invalid_then_corrected(dir.path(), REFERENCE, &invalid, &valid);
}

#[test]
fn commit_missing_second_parent_denies_then_corrected_request_succeeds() {
    let dir = committed_repo();
    let valid = request(dir.path(), &[resolve(dir.path(), REFERENCE)]);
    let mut invalid = valid.clone();
    invalid.parents.push(absent_object(dir.path()));
    invalid_then_corrected(dir.path(), REFERENCE, &invalid, &valid);
}

#[test]
fn commit_tree_as_second_parent_denies_then_corrected_request_succeeds() {
    let dir = committed_repo();
    let valid = request(dir.path(), &[resolve(dir.path(), REFERENCE)]);
    let mut invalid = valid.clone();
    invalid.parents.push(valid.tree);
    invalid_then_corrected(dir.path(), REFERENCE, &invalid, &valid);
}

#[test]
fn commit_blob_as_first_parent_denies_then_corrected_request_succeeds() {
    let dir = committed_repo();
    let valid = request(dir.path(), &[resolve(dir.path(), REFERENCE)]);
    let mut invalid = valid.clone();
    let blob = sdk(dir.path()).write_blob(b"not a parent commit").unwrap().detach();
    invalid.parents[0] = blob;
    // Match the ref to the wrong-kind object so an expected-tip check alone
    // cannot satisfy this test. Write raw metadata only in this corrupt fixture.
    let ref_path = dir.path().join(".git/refs/heads/main");
    fs::write(&ref_path, format!("{blob}\n")).unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let before = without_objects(dir.path());

    assert_git_error(repo.commit(REFERENCE, &invalid));

    assert_eq!(without_objects(dir.path()), before);
    fs::write(ref_path, format!("{}\n", valid.parents[0])).unwrap();
    commit_success(&repo, dir.path(), &valid);
}

#[test]
fn commit_head_alias_denies_then_qualified_branch_succeeds() {
    let dir = committed_repo();
    let commit = request(dir.path(), &[resolve(dir.path(), REFERENCE)]);
    invalid_then_corrected(dir.path(), "HEAD", &commit, &commit);
}

#[test]
fn commit_short_branch_denies_then_qualified_branch_succeeds() {
    let dir = committed_repo();
    let commit = request(dir.path(), &[resolve(dir.path(), REFERENCE)]);
    invalid_then_corrected(dir.path(), "main", &commit, &commit);
}

#[test]
fn commit_tag_reference_denies_then_local_branch_succeeds() {
    let dir = committed_repo();
    let parent = resolve(dir.path(), REFERENCE);
    git(dir.path(), &["update-ref", "refs/tags/keep", &parent.to_string()]);
    let commit = request(dir.path(), &[parent]);
    invalid_then_corrected(dir.path(), "refs/tags/keep", &commit, &commit);
}

#[test]
fn commit_remote_reference_denies_then_local_branch_succeeds() {
    let dir = committed_repo();
    let parent = resolve(dir.path(), REFERENCE);
    git(dir.path(), &["update-ref", "refs/remotes/origin/main", &parent.to_string()]);
    let commit = request(dir.path(), &[parent]);
    invalid_then_corrected(dir.path(), "refs/remotes/origin/main", &commit, &commit);
}

#[test]
fn commit_malformed_branch_denies_then_valid_reference_succeeds() {
    for reference in [
        "refs/heads/",
        "refs/heads/../outside",
        "refs/heads/main.lock",
        "refs/heads/bad\nname",
        "refs/heads/bad\0name",
    ] {
        let dir = committed_repo();
        let commit = request(dir.path(), &[resolve(dir.path(), REFERENCE)]);
        invalid_then_corrected(dir.path(), reference, &commit, &commit);
    }
}

#[test]
fn commit_author_header_injection_denies_then_corrected_identity_succeeds() {
    let dir = committed_repo();
    let valid = request(dir.path(), &[resolve(dir.path(), REFERENCE)]);
    let mut invalid = valid.clone();
    invalid.author.name = "Author\nparent injected".into();
    invalid_then_corrected(dir.path(), REFERENCE, &invalid, &valid);
}

#[test]
fn commit_committer_header_injection_denies_then_corrected_identity_succeeds() {
    let dir = committed_repo();
    let valid = request(dir.path(), &[resolve(dir.path(), REFERENCE)]);
    let mut invalid = valid.clone();
    invalid.committer.email = "mail@example.invalid>\nencoding injected".into();
    invalid_then_corrected(dir.path(), REFERENCE, &invalid, &valid);
}

#[test]
fn commit_empty_author_denies_then_supplied_identity_succeeds() {
    let dir = committed_repo();
    let valid = request(dir.path(), &[resolve(dir.path(), REFERENCE)]);
    let mut invalid = valid.clone();
    invalid.author.name = "".into();
    invalid_then_corrected(dir.path(), REFERENCE, &invalid, &valid);
}

#[test]
fn commit_empty_committer_denies_then_supplied_identity_succeeds() {
    let dir = committed_repo();
    let valid = request(dir.path(), &[resolve(dir.path(), REFERENCE)]);
    let mut invalid = valid.clone();
    invalid.committer.email = "".into();
    invalid_then_corrected(dir.path(), REFERENCE, &invalid, &valid);
}

#[test]
fn commit_encoding_header_injection_denies_then_corrected_header_succeeds() {
    let dir = committed_repo();
    let valid = request(dir.path(), &[resolve(dir.path(), REFERENCE)]);
    let mut invalid = valid.clone();
    invalid.encoding = Some("UTF-8\nparent injected".into());
    invalid_then_corrected(dir.path(), REFERENCE, &invalid, &valid);
}

#[test]
fn commit_extra_header_name_injection_denies_then_corrected_header_succeeds() {
    let dir = committed_repo();
    let valid = request(dir.path(), &[resolve(dir.path(), REFERENCE)]);
    let mut invalid = valid.clone();
    invalid.extra_headers.push(("x-fixture\nparent".into(), "injected".into()));
    invalid_then_corrected(dir.path(), REFERENCE, &invalid, &valid);
}

#[test]
fn commit_duplicate_tree_header_denies_then_corrected_header_succeeds() {
    let dir = committed_repo();
    let valid = request(dir.path(), &[resolve(dir.path(), REFERENCE)]);
    let mut invalid = valid.clone();
    invalid.extra_headers.push(("tree".into(), valid.tree.to_string().into()));
    invalid_then_corrected(dir.path(), REFERENCE, &invalid, &valid);
}
}
