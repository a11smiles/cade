//! Competing writers at deterministic points in the real checkout orchestration.

use super::*;

fn changed_observation(name: &'static str) {
    let dir = fixture();
    dirty_selection(dir.path());
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let original = git(dir.path(), &["rev-parse", name]);
    let replacement = git(
        dir.path(),
        &["rev-parse", if name == "refs/heads/main" { REFERENCE } else { "main" }],
    );
    let path = dir.path().to_path_buf();
    let expected = Rc::new(RefCell::new(None));
    let captured = expected.clone();
    let scope = observer::install(move |phase| {
        if phase != Phase::Observed {
            return;
        }
        assert!(captured.borrow().is_none(), "observation is taken once, not automatically retried");
        if name == "HEAD" {
            git(&path, &["symbolic-ref", "HEAD", REFERENCE]);
        } else {
            git(&path, &["update-ref", name, std::str::from_utf8(&replacement).unwrap().trim()]);
        }
        *captured.borrow_mut() = Some(snapshot(&path));
    });

    let error = assert_git_error(repo.checkout(REFERENCE, actor()));
    drop(scope);

    assert!(error.contains("changed before locks"), "{error}");
    assert_eq!(
        snapshot(dir.path()),
        expected.borrow().as_ref().expect("competitor must run after observation").clone(),
        "only the competitor's own bytes/metadata may change"
    );
    assert_selection(dir.path());
    assert_unlocked(dir.path());
    // Explicit owner correction; no production retry or rollback.
    if name == "HEAD" {
        git(dir.path(), &["symbolic-ref", "HEAD", "refs/heads/main"]);
    } else {
        git(dir.path(), &["update-ref", name, std::str::from_utf8(&original).unwrap().trim()]);
    }
    repo.checkout(REFERENCE, actor()).unwrap();
    assert_target(dir.path());
    assert_selection(dir.path());
}

#[test]
fn checkout_head_changed_after_observation_denies_then_corrected_retry_succeeds() {
    changed_observation("HEAD");
}

#[test]
fn checkout_current_changed_after_observation_denies_then_corrected_retry_succeeds() {
    changed_observation("refs/heads/main");
}

#[test]
fn checkout_target_changed_after_observation_denies_then_corrected_retry_succeeds() {
    changed_observation(REFERENCE);
}

fn guarded_interval(selected: Phase) {
    let dir = fixture();
    dirty_selection(dir.path());
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let index = fs::read(dir.path().join(".git/index")).unwrap();
    let main = git(dir.path(), &["rev-parse", "main"]);
    let topic = git(dir.path(), &["rev-parse", REFERENCE]);
    let path = dir.path().to_path_buf();
    let calls = Rc::new(RefCell::new(0));
    let observed = calls.clone();
    let scope = observer::install(move |phase| {
        if phase != selected {
            return;
        }
        *observed.borrow_mut() += 1;
        let index_lock =
            gix::lock::Marker::acquire_to_hold_resource(path.join(".git/index"), gix::lock::acquire::Fail::Immediately, None, 0);
        assert!(index_lock.is_err(), "real index writer must be excluded at {phase:?}");
        let writer = git2::Repository::open(&path).unwrap();
        for name in ["refs/heads/main", REFERENCE] {
            let mut transaction = writer.transaction().unwrap();
            let error = transaction.lock_ref(name).expect_err("competing branch writer must be excluded");
            assert_eq!(error.code(), git2::ErrorCode::Locked);
        }
        let mut transaction = writer.transaction().unwrap();
        if phase == Phase::HeadPublished {
            // HEAD's publishing transaction is consumed, unlike the independent
            // no-edit branch guards. Acquire/drop only; do not change HEAD.
            transaction.lock_ref("HEAD").expect("own HEAD publication must release its guard");
            assert_eq!(fs::read(path.join(".git/HEAD")).unwrap(), b"ref: refs/heads/topic\n");
        } else {
            let error = transaction.lock_ref("HEAD").expect_err("HEAD must stay guarded until publication");
            assert_eq!(error.code(), git2::ErrorCode::Locked);
            assert_eq!(fs::read(path.join(".git/HEAD")).unwrap(), b"ref: refs/heads/main\n");
        }
        assert_eq!(fs::read(path.join(".git/index")).unwrap(), index, "real index is not published yet");
        assert_eq!(git(&path, &["rev-parse", "main"]), main);
        assert_eq!(git(&path, &["rev-parse", REFERENCE]), topic);
    });

    repo.checkout(REFERENCE, actor()).unwrap();
    drop(scope);

    assert_eq!(*calls.borrow(), 1, "selected orchestration boundary must be reached exactly once");
    assert_target(dir.path());
    assert_selection(dir.path());
    assert_unlocked(dir.path());
}

#[test]
fn checkout_after_guards_excludes_index_current_target_and_head_writers() {
    guarded_interval(Phase::Guarded);
}

#[test]
fn checkout_after_sdk_checkout_excludes_index_current_target_and_head_writers() {
    guarded_interval(Phase::WorktreeUpdated);
}

#[test]
fn checkout_after_head_publication_excludes_index_and_branch_writers_until_index_publication() {
    guarded_interval(Phase::HeadPublished);
}

fn changed_occupancy(selected: Phase) {
    let dir = fixture();
    dirty_selection(dir.path());
    let parent = tempfile::tempdir().unwrap();
    let other = parent.path().join("other");
    git(dir.path(), &["worktree", "add", "-q", "--detach", other.to_str().unwrap(), "main"]);
    dirty_selection(&other);
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let path = dir.path().to_path_buf();
    let competitor = other.clone();
    let expected = Rc::new(RefCell::new(None));
    let captured = expected.clone();
    let phases = Rc::new(RefCell::new(Vec::new()));
    let recorded = phases.clone();
    let scope = observer::install(move |phase| {
        recorded.borrow_mut().push(phase);
        if phase != selected {
            return;
        }
        assert!(
            captured.borrow().is_none(),
            "occupancy observation must not be automatically retried"
        );
        // An external symbolic-HEAD writer need not honor our target guard.
        // The rescan detects this change; it cannot exclude future such writes.
        git(&competitor, &["symbolic-ref", "HEAD", REFERENCE]);
        let mut caller = snapshot(&path);
        for lock in [
            ".git/index.lock",
            ".git/HEAD.lock",
            ".git/refs/heads/main.lock",
            ".git/refs/heads/topic.lock",
        ] {
            caller.remove(Path::new(lock));
        }
        *captured.borrow_mut() = Some((caller, snapshot(&competitor)));
    });

    let error = assert_git_error(repo.checkout(REFERENCE, actor()));
    drop(scope);

    assert_eq!(
        error,
        "checkout target branch is checked out in another worktree; switch or detach that worktree before explicitly retrying"
    );
    let expected = expected.borrow();
    let (caller, competitor) = expected.as_ref().expect("external HEAD writer must run at the selected boundary");
    assert_eq!(&snapshot(dir.path()), caller);
    assert_eq!(&snapshot(&other), competitor);
    assert!(
        !phases.borrow().contains(&Phase::SdkProgress),
        "deny before native worktree mutation"
    );
    assert!(!phases.borrow().contains(&Phase::HeadPublished));
    assert_selection(dir.path());
    assert_selection(&other);
    assert_unlocked(dir.path());

    git(&other, &["checkout", "-q", "--detach"]);
    let released = snapshot(&other);
    repo.checkout(REFERENCE, actor()).unwrap();
    assert_eq!(snapshot(&other), released);
    assert_target(dir.path());
    assert_selection(dir.path());
}

#[test]
fn checkout_occupancy_changed_after_observation_denies_then_owner_detach_allows_retry() {
    changed_occupancy(Phase::Observed);
}

#[test]
fn checkout_occupancy_changed_under_target_guard_denies_then_owner_detach_allows_retry() {
    changed_occupancy(Phase::Guarded);
}
