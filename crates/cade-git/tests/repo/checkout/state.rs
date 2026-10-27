//! Attachment, repository state, real SDK lock contention and explicit retries.

use super::*;

#[test]
fn checkout_packed_target_switches_without_creating_loose_branch_shadow() {
    let dir = fixture();
    git(dir.path(), &["pack-refs", "--all", "--prune"]);
    let packed = fs::read(dir.path().join(".git/packed-refs")).unwrap();
    assert!(!dir.path().join(".git").join(REFERENCE).exists());
    let repo = ProjectRepo::open(dir.path()).unwrap();

    switch(&repo, dir.path());

    assert_eq!(fs::read(dir.path().join(".git/packed-refs")).unwrap(), packed);
    assert!(!dir.path().join(".git").join(REFERENCE).exists());
}

#[test]
fn checkout_packed_target_missing_object_denies_then_restored_object_succeeds() {
    let dir = fixture();
    let target = resolve(dir.path(), REFERENCE);
    git(dir.path(), &["pack-refs", "--all", "--prune"]);
    let object = loose_object(dir.path(), target);
    let saved = fs::read(&object).unwrap();
    fs::remove_file(&object).unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();

    deny_default(&repo, dir.path(), &["commit", "object"]);

    fs::write(object, saved).unwrap();
    switch(&repo, dir.path());
}

#[test]
fn checkout_detached_head_attaches_to_explicit_target_branch() {
    let dir = fixture();
    git(dir.path(), &["checkout", "-q", "--detach"]);
    let repo = ProjectRepo::open(dir.path()).unwrap();

    switch(&repo, dir.path());
}

#[test]
fn checkout_unborn_head_without_index_publishes_existing_target_branch() {
    let dir = fixture();
    git(dir.path(), &["symbolic-ref", "HEAD", "refs/heads/unborn"]);
    fs::remove_file(dir.path().join(".git/index")).unwrap();
    for name in [".gitignore", "keep-stage", "keep-work", "removed.txt", "tracked.txt"] {
        fs::remove_file(dir.path().join(name)).unwrap();
    }
    let repo = ProjectRepo::open(dir.path()).unwrap();
    assert!(!repo.has_commits());

    switch(&repo, dir.path());

    assert!(!dir.path().join(".git/refs/heads/unborn").exists());
}

#[test]
fn checkout_unborn_overlapping_staged_file_denies_without_consuming_selection() {
    let dir = fixture();
    git(dir.path(), &["symbolic-ref", "HEAD", "refs/heads/unborn"]);
    let repo = ProjectRepo::open(dir.path()).unwrap();

    deny_default(&repo, dir.path(), &["staged"]);
}

#[test]
fn checkout_same_branch_preserves_dirty_worktree_and_staged_selection() {
    let dir = fixture();
    git(dir.path(), &["checkout", "-q", "topic"]);
    fs::write(dir.path().join("tracked.txt"), b"same branch staged\n").unwrap();
    git(dir.path(), &["add", "--", "tracked.txt"]);
    fs::write(dir.path().join("tracked.txt"), b"same branch unstaged\n").unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let index = git(dir.path(), &["ls-files", "--stage"]);
    let mut before = snapshot(dir.path());
    before.retain(|path, _| path != Path::new(".git/index") && path != Path::new(".git/logs/HEAD"));

    let result = repo.checkout(REFERENCE, actor());

    assert!(result.is_ok(), "same-branch switch must not discard local selection: {result:?}");
    let mut after = snapshot(dir.path());
    after.retain(|path, _| path != Path::new(".git/index") && path != Path::new(".git/logs/HEAD"));
    assert_eq!(after, before);
    assert_eq!(git(dir.path(), &["ls-files", "--stage"]), index);
}

#[test]
fn checkout_linked_worktree_switches_its_head_and_index_without_changing_primary() {
    let dir = fixture();
    let linked_parent = tempfile::tempdir().unwrap();
    let linked = linked_parent.path().join("linked");
    git(dir.path(), &["worktree", "add", "-q", "--detach", linked.to_str().unwrap(), "main"]);
    let repo = ProjectRepo::open(&linked).unwrap();
    let mut before = snapshot(dir.path());
    before.retain(|path, _| !path.starts_with(".git/worktrees"));
    let link_file = fs::read(linked.join(".git")).unwrap();

    switch(&repo, &linked);

    assert_eq!(fs::read(linked.join(".git")).unwrap(), link_file);
    let mut after = snapshot(dir.path());
    after.retain(|path, _| !path.starts_with(".git/worktrees"));
    assert_eq!(
        after, before,
        "primary worktree/index/HEAD and shared refs/logs/objects must not change"
    );
}

#[test]
fn checkout_bare_repository_denies_without_creating_worktree_or_index() {
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q", "--bare", "-b", "main"]);
    let target = crate::objects::write_commit(dir.path(), &[], "Bare available commit");
    git(dir.path(), &["update-ref", REFERENCE, &target.to_string()]);
    let repo = ProjectRepo::open(dir.path()).unwrap();

    deny_default(&repo, dir.path(), &["working tree", "worktree", "bare"]);
}

#[test]
fn checkout_corrupt_index_denies_then_explicit_repair_succeeds() {
    let dir = fixture();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let path = dir.path().join(".git/index");
    let before = fs::read(&path).unwrap();
    fs::write(&path, b"not a Git index\n").unwrap();

    deny_default(&repo, dir.path(), &["index"]);

    fs::write(path, before).unwrap();
    switch(&repo, dir.path());
}

#[test]
fn checkout_missing_index_on_born_head_denies_instead_of_discarding_staged_deletions() {
    let dir = fixture();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let path = dir.path().join(".git/index");
    let before = fs::read(&path).unwrap();
    fs::remove_file(&path).unwrap();

    deny_default(&repo, dir.path(), &["index", "staged"]);

    fs::write(path, before).unwrap();
    switch(&repo, dir.path());
}

#[test]
fn checkout_corrupt_config_denies_then_explicit_repair_succeeds() {
    let dir = fixture();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let path = dir.path().join(".git/config");
    let before = fs::read(&path).unwrap();
    fs::write(&path, b"[unterminated\n").unwrap();

    deny_default(&repo, dir.path(), &["config"]);

    fs::write(path, before).unwrap();
    switch(&repo, dir.path());
}

#[test]
fn checkout_corrupt_head_denies_then_explicit_repair_succeeds() {
    let dir = fixture();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let path = dir.path().join(".git/HEAD");
    let before = fs::read(&path).unwrap();
    fs::write(&path, b"not a head\n").unwrap();

    deny_default(&repo, dir.path(), &["head", "reference"]);

    fs::write(path, before).unwrap();
    switch(&repo, dir.path());
}

fn locked_resource(resource: &str) {
    let dir = fixture();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let path = dir.path().join(".git").join(resource);
    let lock = gix::lock::Marker::acquire_to_hold_resource(&path, gix::lock::acquire::Fail::Immediately, None, 0).unwrap();
    let lock_path = lock.lock_path().to_path_buf();

    deny_default(&repo, dir.path(), &["lock"]);

    assert!(lock_path.exists(), "checkout must not remove another writer's real lock");
    drop(lock);
    assert!(!lock_path.exists());
    switch(&repo, dir.path());
    assert!(!lock_path.exists());
}

#[test]
fn checkout_real_index_lock_denies_then_owner_release_allows_retry() {
    locked_resource("index");
}

#[test]
fn checkout_real_head_lock_denies_then_owner_release_allows_retry() {
    locked_resource("HEAD");
}

#[test]
fn checkout_real_current_branch_lock_denies_then_owner_release_allows_retry() {
    locked_resource("refs/heads/main");
}

#[test]
fn checkout_real_target_branch_lock_denies_then_owner_release_allows_retry() {
    locked_resource(REFERENCE);
}

#[test]
fn checkout_linked_worktree_index_lock_denies_then_owner_release_allows_retry() {
    let dir = fixture();
    let linked_parent = tempfile::tempdir().unwrap();
    let linked = linked_parent.path().join("linked");
    git(dir.path(), &["worktree", "add", "-q", "--detach", linked.to_str().unwrap(), "main"]);
    let repo = ProjectRepo::open(&linked).unwrap();
    let index = sdk(&linked).index_path();
    let lock = gix::lock::Marker::acquire_to_hold_resource(index, gix::lock::acquire::Fail::Immediately, None, 0).unwrap();
    let common_before = snapshot(dir.path());

    deny_default(&repo, &linked, &["lock"]);

    assert_eq!(
        snapshot(dir.path()),
        common_before,
        "linked denial must preserve the common git directory"
    );
    drop(lock);
    switch(&repo, &linked);
}

fn operation_marker(name: &str, directory: bool, categories: &[&str]) {
    let dir = fixture();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let marker = dir.path().join(".git").join(name);
    if directory {
        fs::create_dir(&marker).unwrap();
        fs::write(marker.join("head"), format!("{}\n", resolve(dir.path(), "HEAD"))).unwrap();
    } else {
        fs::write(&marker, format!("{}\n", resolve(dir.path(), "HEAD"))).unwrap();
    }

    deny_default(&repo, dir.path(), categories);

    if directory {
        fs::remove_dir_all(marker).unwrap();
    } else {
        fs::remove_file(marker).unwrap();
    }
    switch(&repo, dir.path());
}

#[test]
fn checkout_unresolved_merge_denies_then_explicit_state_correction_succeeds() {
    operation_marker("MERGE_HEAD", false, &["merge", "unresolved", "operation"]);
}

#[test]
fn checkout_unresolved_cherry_pick_denies_then_explicit_state_correction_succeeds() {
    operation_marker("CHERRY_PICK_HEAD", false, &["cherry", "unresolved", "operation"]);
}

#[test]
fn checkout_unresolved_revert_denies_then_explicit_state_correction_succeeds() {
    operation_marker("REVERT_HEAD", false, &["revert", "unresolved", "operation"]);
}

#[test]
fn checkout_unresolved_rebase_merge_denies_then_explicit_state_correction_succeeds() {
    operation_marker("rebase-merge", true, &["rebase", "unresolved", "operation"]);
}

#[test]
fn checkout_unresolved_rebase_apply_denies_then_explicit_state_correction_succeeds() {
    operation_marker("rebase-apply", true, &["rebase", "apply", "unresolved", "operation"]);
}

#[test]
fn checkout_unresolved_sequencer_denies_then_explicit_state_correction_succeeds() {
    operation_marker("sequencer", true, &["sequencer", "unresolved", "operation"]);
}

#[test]
fn checkout_unmerged_index_denies_without_resolving_conflict_stages() {
    let dir = fixture();
    let ancestor = resolve(dir.path(), "main");
    fs::write(dir.path().join("tracked.txt"), b"conflicting main\n").unwrap();
    git(dir.path(), &["add", "--", "tracked.txt"]);
    git(dir.path(), &["commit", "-q", "-m", "Conflicting main"]);
    git(dir.path(), &["read-tree", "-m", &ancestor.to_string(), "main", "topic"]);
    let unmerged = git(dir.path(), &["ls-files", "--unmerged"]);
    assert!(!unmerged.is_empty(), "fixture must contain real conflict stages");
    let repo = ProjectRepo::open(dir.path()).unwrap();

    deny_default(&repo, dir.path(), &["unmerged", "conflict", "unresolved"]);
}

#[test]
fn checkout_known_reflog_obstruction_denies_before_any_content_write() {
    let dir = fixture();
    let log = dir.path().join(".git/logs/HEAD");
    fs::remove_file(&log).unwrap();
    fs::create_dir(&log).unwrap();
    fs::write(log.join("user-entry"), b"preserve obstruction\n").unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();

    deny_default(&repo, dir.path(), &["reflog", "logs", "head"]);
}
