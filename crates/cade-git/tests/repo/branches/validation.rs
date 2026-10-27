//! Known denials must leave even empty directories and foreign locks intact.

use super::*;
use crate::commit::validation::absent_object;

isolated_tests! {
fn invalid_reference_then_corrected(reference: &str) {
    let dir = committed_repo();
    let target = resolve(dir.path(), "HEAD");
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let before = snapshot(dir.path());

    assert_git_error(repo.create_branch(reference, target, committer()));

    assert_eq!(snapshot(dir.path()), before);
    branch_success(&repo, dir.path(), REFERENCE, target);
}

fn invalid_target_then_corrected(dir: &Path, invalid: gix::ObjectId) {
    let valid = resolve(dir, "HEAD");
    let repo = ProjectRepo::open(dir).unwrap();
    let before = snapshot(dir);

    assert_git_error(repo.create_branch(REFERENCE, invalid, committer()));

    assert_eq!(snapshot(dir), before);
    branch_success(&repo, dir, REFERENCE, valid);
}

#[test]
fn create_branch_head_alias_denies_then_full_reference_succeeds() {
    invalid_reference_then_corrected("HEAD");
}

#[test]
fn create_branch_short_name_denies_then_full_reference_succeeds() {
    invalid_reference_then_corrected("topic");
}

#[test]
fn create_branch_nonlocal_namespace_denies_then_local_reference_succeeds() {
    for reference in ["refs/tags/topic", "refs/remotes/origin/topic"] {
        invalid_reference_then_corrected(reference);
    }
}

#[test]
fn create_branch_malformed_name_denies_then_corrected_reference_succeeds() {
    for reference in [
        "refs/heads/",
        "refs/heads/../outside",
        "refs/heads/topic.lock",
        "refs/heads/bad\nname",
        "refs/heads/bad\0name",
        "refs/heads/space name",
        "refs/heads/double..dot",
    ] {
        invalid_reference_then_corrected(reference);
    }
}

#[test]
fn create_branch_missing_target_denies_then_available_commit_succeeds() {
    let dir = committed_repo();
    invalid_target_then_corrected(dir.path(), absent_object(dir.path()));
}

#[test]
fn create_branch_blob_target_denies_then_commit_succeeds() {
    let dir = committed_repo();
    let blob = sdk(dir.path()).write_blob(b"not a commit").unwrap().detach();
    invalid_target_then_corrected(dir.path(), blob);
}

#[test]
fn create_branch_tree_target_denies_then_commit_succeeds() {
    let dir = committed_repo();
    invalid_target_then_corrected(dir.path(), resolve(dir.path(), "HEAD^{tree}"));
}

#[test]
fn create_branch_annotated_tag_target_denies_without_peeling_then_commit_succeeds() {
    let dir = committed_repo();
    git(dir.path(), &["tag", "-a", "fixture", "-m", "Annotated fixture"]);
    invalid_target_then_corrected(dir.path(), resolve(dir.path(), "refs/tags/fixture"));
}

#[test]
fn create_branch_malformed_commit_payload_denies_then_valid_commit_succeeds() {
    let dir = committed_repo();
    let invalid = malformed_commit(dir.path());
    invalid_target_then_corrected(dir.path(), invalid);
}

#[test]
fn create_branch_corrupt_loose_object_denies_then_repaired_object_succeeds() {
    let dir = committed_repo();
    let target = write_commit(dir.path(), &[], "Repairable object");
    let path = loose_object(dir.path(), target);
    let bytes = fs::read(&path).unwrap();
    fs::remove_file(&path).unwrap();
    fs::write(&path, b"not compressed Git object bytes").unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let before = snapshot(dir.path());

    assert_git_error(repo.create_branch(REFERENCE, target, committer()));

    assert_eq!(snapshot(dir.path()), before);
    fs::write(path, bytes).unwrap();
    branch_success(&repo, dir.path(), REFERENCE, target);
}

#[test]
fn create_branch_invalid_identity_denies_then_explicit_actor_succeeds() {
    for field in [
        b"".as_slice(),
        b" ",
        b"bad\nname",
        b"bad\rname",
        b"bad\0name",
        b"bad<name",
        b"bad>name",
        b"actor-input-canary\n",
    ] {
        for is_name in [true, false] {
            let dir = committed_repo();
            let target = resolve(dir.path(), "HEAD");
            let repo = ProjectRepo::open(dir.path()).unwrap();
            let mut invalid = committer();
            if is_name {
                invalid.name = field.as_bstr();
            } else {
                invalid.email = field.as_bstr();
            }
            let before = snapshot(dir.path());

            let result = repo.create_branch(REFERENCE, target, invalid);
            let error = result.as_ref().expect_err("invalid identity must be denied");
            let diagnostics = [error.to_string(), format!("{error:?}"), format!("{error:#?}")];
            assert_git_error(result);
            for diagnostic in diagnostics {
                assert!(
                    !diagnostic.contains("actor-input-canary"),
                    "invalid actor diagnostic must not disclose the supplied identity"
                );
            }

            assert_eq!(snapshot(dir.path()), before);
            branch_success(&repo, dir.path(), REFERENCE, target);
        }
    }

    // Keep the identity cases above: malformed raw time takes a separate SDK
    // parse-error path before the owned signature's identity validation.
    let dir = committed_repo();
    let target = resolve(dir.path(), "HEAD");
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let invalid = gix::actor::SignatureRef {
        time: "actor-input-canary",
        ..committer()
    };
    let before = snapshot(dir.path());

    let result = repo.create_branch(REFERENCE, target, invalid);
    let error = result.as_ref().expect_err("invalid actor time must be denied");
    let diagnostics = [error.to_string(), format!("{error:?}"), format!("{error:#?}")];
    assert_git_error(result);
    for diagnostic in diagnostics {
        assert!(
            !diagnostic.contains("actor-input-canary"),
            "invalid actor time diagnostic must not disclose the supplied timestamp"
        );
    }

    assert_eq!(snapshot(dir.path()), before);
    branch_success(&repo, dir.path(), REFERENCE, target);
}

#[test]
fn create_branch_invalid_actor_time_denies_then_explicit_actor_succeeds() {
    for time in ["", "not a time", "946688400 +9999", "946688400 +0530\ninjected"] {
        let dir = committed_repo();
        let target = resolve(dir.path(), "HEAD");
        let repo = ProjectRepo::open(dir.path()).unwrap();
        let invalid = gix::actor::SignatureRef { time, ..committer() };
        let before = snapshot(dir.path());

        assert_git_error(repo.create_branch(REFERENCE, target, invalid));

        assert_eq!(snapshot(dir.path()), before);
        branch_success(&repo, dir.path(), REFERENCE, target);
    }
}

#[test]
fn create_branch_invalid_target_preflight_preserves_empty_ref_directory() {
    let dir = committed_repo();
    let reference = "refs/heads/empty/topic";
    fs::create_dir(dir.path().join(".git/refs/heads/empty")).unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let before = snapshot(dir.path());

    assert_git_error(repo.create_branch(reference, absent_object(dir.path()), committer()));

    assert_eq!(snapshot(dir.path()), before);
    branch_success(&repo, dir.path(), reference, resolve(dir.path(), "HEAD"));
}
}
