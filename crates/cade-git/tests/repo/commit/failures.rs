//! Lock, stale-tip, concurrency and recovery probes against real local refs.

use super::*;

isolated_tests! {
fn locked_then_retried(relative_lock: &str, resource: &str) {
    let dir = committed_repo();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let commit = request(dir.path(), &[resolve(dir.path(), REFERENCE)]);
    dirty_selection(dir.path());
    let lock = dir.path().join(relative_lock);
    fs::write(&lock, b"owned by another writer\n").unwrap();
    let before = without_objects(dir.path());

    let message = assert_git_error(repo.commit(REFERENCE, &commit));

    assert!(
        message.to_lowercase().contains("lock"),
        "lock error must identify obstruction: {message}"
    );
    assert!(
        message.contains(resource),
        "lock error must identify the locked resource: {message}"
    );
    assert_eq!(
        without_objects(dir.path()),
        before,
        "failure must not consume another writer's lock"
    );
    fs::remove_file(lock).unwrap();
    let before_retry = protected_state(dir.path());
    commit_success(&repo, dir.path(), &commit);
    assert_eq!(protected_state(dir.path()), before_retry);
}

#[test]
fn commit_index_lock_denies_then_explicit_retry_succeeds() {
    locked_then_retried(".git/index.lock", "index");
}

#[test]
fn commit_ref_lock_denies_then_explicit_retry_succeeds() {
    locked_then_retried(".git/refs/heads/main.lock", "main");
}

#[test]
fn commit_initial_index_lock_denies_without_creating_index_then_retry_succeeds() {
    let dir = initial_repo();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let commit = request(dir.path(), &[]);
    let lock = dir.path().join(".git/index.lock");
    fs::write(&lock, b"another writer\n").unwrap();
    let before = without_objects(dir.path());

    let message = assert_git_error(repo.commit(REFERENCE, &commit));

    assert!(message.contains("index"), "index obstruction must be identifiable: {message}");
    assert_eq!(without_objects(dir.path()), before);
    fs::remove_file(lock).unwrap();
    commit_success(&repo, dir.path(), &commit);
    assert!(!dir.path().join(".git/index").exists());
}

#[test]
fn commit_changed_tip_denies_then_corrected_parent_succeeds() {
    for packed in [false, true] {
        let dir = committed_repo();
        let repo = ProjectRepo::open(dir.path()).unwrap();
        let mut commit = request(dir.path(), &[resolve(dir.path(), REFERENCE)]);
        let mut competing = commit.clone();
        competing.message = "Another writer's commit\n".into();
        let competing_id = sdk(dir.path()).write_object(&competing).unwrap().detach();
        git(
            dir.path(),
            &["update-ref", REFERENCE, &competing_id.to_string(), &commit.parents[0].to_string()],
        );
        if packed {
            git(dir.path(), &["pack-refs", "--all", "--prune"]);
            let heads = dir.path().join(".git/refs/heads");
            fs::create_dir_all(&heads).unwrap();
            assert!(
                fs::read_dir(&heads).unwrap().next().is_none(),
                "packed fixture must have an empty refs/heads"
            );
            assert!(dir.path().join(".git/packed-refs").is_file());
            assert_eq!(resolve(dir.path(), REFERENCE), competing_id);
        }
        let before = without_objects(dir.path());

        assert_git_error(repo.commit(REFERENCE, &commit));

        assert_eq!(resolve(dir.path(), REFERENCE), competing_id);
        assert_eq!(
            without_objects(dir.path()),
            before,
            "denied stale-parent update must preserve all non-object state, including empty .git/refs/heads (packed={packed})"
        );
        commit.parents[0] = competing_id;
        commit_success(&repo, dir.path(), &commit);
    }
}

#[test]
fn commit_deleted_tip_denies_without_recreating_branch_then_initial_request_succeeds() {
    let dir = committed_repo();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let mut commit = request(dir.path(), &[resolve(dir.path(), REFERENCE)]);
    git(dir.path(), &["update-ref", "-d", REFERENCE, &commit.parents[0].to_string()]);
    let before = without_objects(dir.path());

    assert_git_error(repo.commit(REFERENCE, &commit));

    assert!(!dir.path().join(".git/refs/heads/main").exists());
    assert_eq!(without_objects(dir.path()), before);
    commit.parents.clear();
    commit_success(&repo, dir.path(), &commit);
}

#[test]
fn commit_symbolic_target_denies_without_dereferencing_then_direct_target_succeeds() {
    let dir = committed_repo();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let parent = resolve(dir.path(), REFERENCE);
    let commit = request(dir.path(), &[parent]);
    git(dir.path(), &["update-ref", "refs/heads/other", &parent.to_string()]);
    git(dir.path(), &["symbolic-ref", REFERENCE, "refs/heads/other"]);
    let before = without_objects(dir.path());

    assert_git_error(repo.commit(REFERENCE, &commit));

    assert_eq!(git(dir.path(), &["symbolic-ref", REFERENCE]), b"refs/heads/other\n");
    assert_eq!(resolve(dir.path(), "refs/heads/other"), parent);
    assert_eq!(without_objects(dir.path()), before);
    git(dir.path(), &["update-ref", "--no-deref", REFERENCE, &parent.to_string()]);
    commit_success(&repo, dir.path(), &commit);
    assert_eq!(resolve(dir.path(), "refs/heads/other"), parent);
}

#[test]
fn commit_initial_symbolic_target_denies_without_creating_destination_then_retry_succeeds() {
    let dir = initial_repo();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let commit = request(dir.path(), &[]);
    git(dir.path(), &["symbolic-ref", REFERENCE, "refs/heads/missing"]);
    let before = without_objects(dir.path());

    assert_git_error(repo.commit(REFERENCE, &commit));

    assert_eq!(without_objects(dir.path()), before);
    assert!(!dir.path().join(".git/refs/heads/missing").exists());
    fs::remove_file(dir.path().join(".git/refs/heads/main")).unwrap();
    commit_success(&repo, dir.path(), &commit);
    assert!(!dir.path().join(".git/refs/heads/missing").exists());
}

#[test]
fn commit_initial_existing_target_denies_then_parented_request_succeeds() {
    let dir = committed_repo();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let parent = resolve(dir.path(), REFERENCE);
    let mut commit = request(dir.path(), &[]);
    let before = without_objects(dir.path());

    assert_git_error(repo.commit(REFERENCE, &commit));

    assert_eq!(without_objects(dir.path()), before);
    commit.parents.push(parent);
    commit_success(&repo, dir.path(), &commit);
}

#[test]
fn commit_competing_same_parent_requests_publish_exactly_one_winner() {
    let dir = committed_repo();
    let parent = resolve(dir.path(), REFERENCE);
    let first = request(dir.path(), &[parent]);
    let mut second = first.clone();
    second.message = "Competing request\n".into();
    dirty_selection(dir.path());
    let before = protected_state(dir.path());
    let first_repo = ProjectRepo::open(dir.path()).unwrap();
    let second_repo = ProjectRepo::open(dir.path()).unwrap();
    let barrier = std::sync::Barrier::new(3);

    let (a, b) = std::thread::scope(|scope| {
        let barrier = &barrier;
        let first = &first;
        let second = &second;
        let a = scope.spawn(move || {
            barrier.wait();
            first_repo.commit(REFERENCE, first)
        });
        let b = scope.spawn(move || {
            barrier.wait();
            second_repo.commit(REFERENCE, second)
        });
        barrier.wait();
        (a.join().unwrap(), b.join().unwrap())
    });

    assert_eq!(
        usize::from(a.is_ok()) + usize::from(b.is_ok()),
        1,
        "same-parent competitors must produce exactly one success: {a:?}, {b:?}"
    );
    let (id, expected) = match (a, b) {
        (Ok(id), denied @ Err(_)) => {
            assert_git_error(denied);
            (id, &first)
        }
        (denied @ Err(_), Ok(id)) => {
            assert_git_error(denied);
            (id, &second)
        }
        _ => unreachable!("exactly one success was asserted"),
    };
    assert_stored(dir.path(), expected, id);
    assert_eq!(git(dir.path(), &["rev-list", "--count", REFERENCE]), b"2\n");
    let reflog = git(dir.path(), &["reflog", "show", "--format=%H", REFERENCE]);
    assert_eq!(String::from_utf8(reflog).unwrap().lines().count(), 2);
    assert_eq!(protected_state(dir.path()), before);
}

#[test]
fn commit_completed_request_retry_denies_without_duplicate_commit() {
    let dir = committed_repo();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let commit = request(dir.path(), &[resolve(dir.path(), REFERENCE)]);
    let id = commit_success(&repo, dir.path(), &commit);
    let before = snapshot(dir.path());

    assert_git_error(repo.commit(REFERENCE, &commit));

    assert_eq!(resolve(dir.path(), REFERENCE), id);
    assert_eq!(git(dir.path(), &["rev-list", "--count", REFERENCE]), b"2\n");
    assert_eq!(snapshot(dir.path()), before);
}

#[test]
fn commit_completed_initial_request_retry_denies_without_duplicate_commit() {
    let dir = initial_repo();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let commit = request(dir.path(), &[]);
    let id = commit_success(&repo, dir.path(), &commit);
    let before = snapshot(dir.path());

    assert_git_error(repo.commit(REFERENCE, &commit));

    assert_eq!(resolve(dir.path(), REFERENCE), id);
    assert_eq!(git(dir.path(), &["rev-list", "--count", REFERENCE]), b"1\n");
    assert_eq!(snapshot(dir.path()), before);
}

// Unix mode-bit fixtures require a nonroot runner and deliberately fail if the
// denial cannot be demonstrated. Windows ACL coverage remains a separate lane.
#[cfg(unix)]
#[test]
fn commit_unwritable_ref_directory_denies_then_permission_repair_succeeds() {
    use std::os::unix::fs::PermissionsExt;

    let dir = committed_repo();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let commit = request(dir.path(), &[resolve(dir.path(), REFERENCE)]);
    let before = without_objects(dir.path());
    let path = dir.path().join(".git/refs/heads");
    let original = fs::metadata(&path).unwrap().permissions();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o500)).unwrap();
    let probe_path = path.join("permission-probe");
    let probe = fs::write(&probe_path, b"must be denied");
    let result = repo.commit(REFERENCE, &commit);
    // Restore before asserting so the temporary directory remains removable.
    fs::set_permissions(path, original).unwrap();
    if probe.is_ok() {
        fs::remove_file(probe_path).unwrap();
    }

    assert_eq!(probe.unwrap_err().kind(), std::io::ErrorKind::PermissionDenied);
    let message = assert_git_error(result);
    assert!(
        message.to_lowercase().contains("permission") || message.to_lowercase().contains("denied"),
        "permission failure must retain its cause: {message}"
    );
    assert_eq!(without_objects(dir.path()), before);
    commit_success(&repo, dir.path(), &commit);
}

#[cfg(unix)]
#[test]
fn commit_unreadable_selected_object_denies_then_permission_repair_succeeds() {
    use std::os::unix::fs::PermissionsExt;

    let dir = committed_repo();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let commit = request(dir.path(), &[resolve(dir.path(), REFERENCE)]);
    let hex = commit.tree.to_string();
    let path = dir.path().join(".git/objects").join(&hex[..2]).join(&hex[2..]);
    let original = fs::metadata(&path).unwrap().permissions();
    let before = without_objects(dir.path());
    fs::set_permissions(&path, fs::Permissions::from_mode(0o0)).unwrap();
    let probe = fs::read(&path);
    let result = repo.commit(REFERENCE, &commit);
    fs::set_permissions(path, original).unwrap();

    assert_eq!(probe.unwrap_err().kind(), std::io::ErrorKind::PermissionDenied);
    let message = assert_git_error(result);
    assert!(
        message.to_lowercase().contains("permission") || message.to_lowercase().contains("denied"),
        "object read failure must retain its cause: {message}"
    );
    assert_eq!(without_objects(dir.path()), before);
    commit_success(&repo, dir.path(), &commit);
}

#[cfg(unix)]
#[test]
fn commit_configured_external_programs_preserves_selection_without_executing_them() {
    use std::os::unix::fs::PermissionsExt;

    let dir = committed_repo();
    let commit = request(dir.path(), &[resolve(dir.path(), REFERENCE)]);
    let hooks = dir.path().join(".git/hooks");
    fs::create_dir_all(&hooks).unwrap();
    let script = b"#!/bin/sh\ntouch \"$0.ran\"\nexit 91\n";
    for name in [
        "pre-commit",
        "prepare-commit-msg",
        "commit-msg",
        "post-commit",
        "signer",
        "filter",
        "ssh",
    ] {
        let path = hooks.join(name);
        fs::write(&path, script).unwrap();
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
    let signer = hooks.join("signer");
    let filter = hooks.join("filter");
    let ssh = hooks.join("ssh");
    git(dir.path(), &["config", "core.hooksPath", hooks.to_str().unwrap()]);
    git(dir.path(), &["config", "commit.gpgSign", "true"]);
    git(dir.path(), &["config", "gpg.program", signer.to_str().unwrap()]);
    git(dir.path(), &["config", "core.sshCommand", ssh.to_str().unwrap()]);
    git(dir.path(), &["config", "remote.origin.url", "ssh://example.invalid/fixture"]);
    for kind in ["clean", "smudge", "process"] {
        git(
            dir.path(),
            &["config", &format!("filter.sentinel.{kind}"), filter.to_str().unwrap()],
        );
    }
    fs::write(dir.path().join(".gitattributes"), b"*.cade filter=sentinel\n").unwrap();
    fs::write(dir.path().join("selected.cade"), b"dirty content must not be filtered\n").unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let before = protected_state(dir.path());

    commit_success(&repo, dir.path(), &commit);

    assert_eq!(protected_state(dir.path()), before, "no hook/filter/signer/SSH sentinel may run");
    assert!(
        !stored_commit(dir.path(), resolve(dir.path(), REFERENCE))
            .extra_headers
            .iter()
            .any(|(name, _)| name == "gpgsig"),
        "local commit must not invoke signing"
    );
}
}
