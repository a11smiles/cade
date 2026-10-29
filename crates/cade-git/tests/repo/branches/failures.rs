//! Publication races and filesystem obstructions, without automatic retries.

use super::*;

isolated_tests! {
fn duplicate_then_corrected(dir: &Path, target: gix::ObjectId) {
    let repo = ProjectRepo::open(dir).unwrap();
    let before = snapshot(dir);

    assert_git_error(repo.create_branch(REFERENCE, target, committer()));

    assert_eq!(snapshot(dir), before, "duplicate creation must not overwrite or append a reflog");
    branch_success(&repo, dir, "refs/heads/corrected", target);
}

#[test]
fn create_branch_existing_different_target_denies_without_overwriting_then_new_name_succeeds() {
    let dir = committed_repo();
    let existing = resolve(dir.path(), "HEAD");
    git(dir.path(), &["update-ref", REFERENCE, &existing.to_string()]);
    let target = write_commit(dir.path(), &[existing], "Different target");
    duplicate_then_corrected(dir.path(), target);
}

#[test]
fn create_branch_existing_identical_target_denies_without_false_creation_success() {
    let dir = committed_repo();
    let target = resolve(dir.path(), "HEAD");
    git(dir.path(), &["update-ref", REFERENCE, &target.to_string()]);
    duplicate_then_corrected(dir.path(), target);
}

#[test]
fn create_branch_packed_duplicate_denies_without_shadowing_then_new_name_succeeds() {
    for identical in [true, false] {
        let dir = committed_repo();
        let existing = resolve(dir.path(), "HEAD");
        let target = if identical {
            existing
        } else {
            write_commit(dir.path(), &[existing], "Different packed target")
        };
        let repo = ProjectRepo::open(dir.path()).unwrap();
        git(dir.path(), &["update-ref", REFERENCE, &existing.to_string()]);
        git(dir.path(), &["pack-refs", "--all", "--prune"]);
        assert!(!dir.path().join(".git").join(REFERENCE).exists());
        let before = snapshot(dir.path());

        assert_git_error(repo.create_branch(REFERENCE, target, committer()));

        assert_eq!(snapshot(dir.path()), before);
        branch_success(&repo, dir.path(), "refs/heads/corrected", target);
    }
}

#[test]
fn create_branch_symbolic_duplicate_denies_without_dereferencing_then_new_name_succeeds() {
    for destination in ["refs/heads/main", "refs/heads/missing"] {
        let dir = committed_repo();
        let target = resolve(dir.path(), "HEAD");
        git(dir.path(), &["symbolic-ref", REFERENCE, destination]);
        duplicate_then_corrected(dir.path(), target);
    }
}

#[test]
fn create_branch_known_duplicate_preflight_reports_existing_ref_before_lock_obstruction() {
    let dir = committed_repo();
    let target = resolve(dir.path(), "HEAD");
    git(dir.path(), &["update-ref", REFERENCE, &target.to_string()]);
    let lock = dir.path().join(".git/refs/heads/topic.lock");
    fs::write(&lock, b"another writer owns this lock\n").unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let before = snapshot(dir.path());

    let message = assert_git_error(repo.create_branch(REFERENCE, target, committer()));

    assert!(
        message.to_lowercase().contains("exist"),
        "known duplicates must be rejected before acquiring locks: {message}"
    );
    assert_eq!(snapshot(dir.path()), before);
}

#[test]
fn create_branch_ref_lock_denies_then_explicit_corrected_retry_succeeds() {
    let dir = committed_repo();
    let target = resolve(dir.path(), "HEAD");
    let lock = dir.path().join(".git/refs/heads/topic.lock");
    fs::write(&lock, b"foreign lock bytes\n").unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let before = snapshot(dir.path());

    let message = assert_git_error(repo.create_branch(REFERENCE, target, committer()));

    assert!(
        message.to_lowercase().contains("lock"),
        "lock failures must retain their cause: {message}"
    );
    assert_eq!(snapshot(dir.path()), before);
    fs::remove_file(lock).unwrap();
    branch_success(&repo, dir.path(), REFERENCE, target);
}

fn competing_creators(identical: bool) {
    let dir = committed_repo();
    let first = resolve(dir.path(), "HEAD");
    let second = if identical {
        first
    } else {
        write_commit(dir.path(), &[first], "Competing target")
    };
    dirty_selection(dir.path());
    let before = snapshot(dir.path());
    let first_repo = ProjectRepo::open(dir.path()).unwrap();
    let second_repo = ProjectRepo::open(dir.path()).unwrap();
    let barrier = std::sync::Barrier::new(3);

    let (a, b) = std::thread::scope(|scope| {
        let barrier = &barrier;
        let a = scope.spawn(move || {
            barrier.wait();
            first_repo.create_branch(REFERENCE, first, committer())
        });
        let b = scope.spawn(move || {
            barrier.wait();
            second_repo.create_branch(REFERENCE, second, committer())
        });
        barrier.wait();
        (a.join().unwrap(), b.join().unwrap())
    });

    assert_eq!(
        usize::from(a.is_ok()) + usize::from(b.is_ok()),
        1,
        "exactly one creator must succeed even for an identical target: {a:?}, {b:?}"
    );
    let winner = match (a, b) {
        (Ok(()), denied @ Err(_)) => {
            assert_git_error(denied);
            first
        }
        (denied @ Err(_), Ok(())) => {
            assert_git_error(denied);
            second
        }
        _ => unreachable!("exactly one success was asserted"),
    };
    assert_eq!(resolve(dir.path(), REFERENCE), winner);
    let log = fs::read_to_string(dir.path().join(".git/logs").join(REFERENCE)).unwrap();
    assert_eq!(log.lines().count(), 1, "the loser must not append a reflog");
    let mut after = snapshot(dir.path());
    // Both parent directories were already present in the committed fixture.
    assert!(after.remove(Path::new(".git/refs/heads/topic")).is_some());
    assert!(after.remove(Path::new(".git/logs/refs/heads/topic")).is_some());
    assert_eq!(after, before);
}

#[test]
fn create_branch_competing_different_targets_publishes_exactly_one_without_overwrite() {
    competing_creators(false);
}

#[test]
fn create_branch_competing_identical_targets_reports_exactly_one_success() {
    competing_creators(true);
}

#[test]
fn create_branch_completed_request_retry_denies_without_extra_reflog_or_mutation() {
    let dir = committed_repo();
    let target = resolve(dir.path(), "HEAD");
    let repo = ProjectRepo::open(dir.path()).unwrap();
    branch_success(&repo, dir.path(), REFERENCE, target);
    let before = snapshot(dir.path());

    assert_git_error(repo.create_branch(REFERENCE, target, committer()));

    assert_eq!(snapshot(dir.path()), before);
}

// Require a real denial on a nonroot Unix runner; never silently skip the case.
// Windows ACL execution remains a separate platform obligation.
#[cfg(unix)]
#[test]
fn create_branch_unwritable_ref_directory_denies_then_permission_repair_succeeds() {
    use std::os::unix::fs::PermissionsExt;

    let dir = committed_repo();
    let target = resolve(dir.path(), "HEAD");
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let before = snapshot(dir.path());
    let path = dir.path().join(".git/refs/heads");
    let original = fs::metadata(&path).unwrap().permissions();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o500)).unwrap();
    let probe_path = path.join("permission-probe");
    let probe = fs::write(&probe_path, b"must be denied");
    let result = repo.create_branch(REFERENCE, target, committer());
    fs::set_permissions(path, original).unwrap();
    if probe.is_ok() {
        fs::remove_file(probe_path).unwrap();
    }

    assert_eq!(probe.unwrap_err().kind(), std::io::ErrorKind::PermissionDenied);
    let message = assert_git_error(result);
    assert!(message.to_lowercase().contains("permission") || message.to_lowercase().contains("denied"));
    assert_eq!(snapshot(dir.path()), before);
    branch_success(&repo, dir.path(), REFERENCE, target);
}

#[cfg(unix)]
#[test]
fn create_branch_unreadable_target_denies_then_permission_repair_succeeds() {
    use std::os::unix::fs::PermissionsExt;

    let dir = committed_repo();
    let target = write_commit(dir.path(), &[], "Unreadable target");
    let path = loose_object(dir.path(), target);
    let original = fs::metadata(&path).unwrap().permissions();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let before = snapshot(dir.path());
    fs::set_permissions(&path, fs::Permissions::from_mode(0o0)).unwrap();
    let probe = fs::read(&path);
    let result = repo.create_branch(REFERENCE, target, committer());
    fs::set_permissions(path, original).unwrap();

    assert_eq!(probe.unwrap_err().kind(), std::io::ErrorKind::PermissionDenied);
    let message = assert_git_error(result);
    assert!(message.to_lowercase().contains("permission") || message.to_lowercase().contains("denied"));
    assert_eq!(snapshot(dir.path()), before);
    branch_success(&repo, dir.path(), REFERENCE, target);
}
}
