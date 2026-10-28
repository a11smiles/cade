//! Ordinary Git branch occupancy: deny sharing, then retry after explicit detach.

use super::*;

isolated_tests! {
fn occupied_target(caller: &Path, other: &Path, reference: &str) {
    let repo = ProjectRepo::open(caller).unwrap();
    fs::write(other.join("keep-stage"), b"other worktree staged selection\n").unwrap();
    git(other, &["add", "--", "keep-stage"]);
    fs::write(other.join("keep-stage"), b"other worktree unstaged selection\n").unwrap();
    fs::write(other.join("keep-work"), b"other worktree tracked edit\n").unwrap();
    fs::write(other.join("keep-untracked"), b"other worktree untracked bytes\n").unwrap();
    fs::write(other.join("keep-ignored"), b"other worktree ignored bytes\n").unwrap();
    let other_repo = sdk(other);
    let other_before = snapshot(other);
    let metadata_before = snapshot(other_repo.git_dir());
    let caller_before = snapshot(caller);
    let target = resolve(caller, reference);

    let result = repo.checkout(reference, actor());

    assert_eq!(snapshot(other), other_before, "occupied worktree bytes must be preserved");
    assert_eq!(
        snapshot(other_repo.git_dir()),
        metadata_before,
        "occupied worktree HEAD/index must be preserved"
    );
    assert_eq!(snapshot(caller), caller_before, "occupancy denial must precede caller writes");
    assert_category(&assert_git_error(result), &["worktree", "working tree", "checked out"]);

    // The owner releases its attachment, not its edits. No force checkout,
    // reset, clean or production repair is permitted.
    git(other, &["checkout", "-q", "--detach"]);
    let metadata_is_directory = other.join(".git").is_dir();
    let mut other_before = snapshot(other);
    other_before.retain(|path, _| !metadata_is_directory || !path.starts_with(".git"));
    let head_before = fs::read(other_repo.git_dir().join("HEAD")).unwrap();
    let index_before = fs::read(other_repo.index_path()).unwrap();

    let result = repo.checkout(reference, actor());

    assert!(result.is_ok(), "explicit detach must permit a corrected retry: {result:?}");
    assert_eq!(git(caller, &["symbolic-ref", "HEAD"]), format!("{reference}\n").as_bytes());
    assert_eq!(resolve(caller, "HEAD"), target);
    assert_eq!(resolve(caller, reference), target);
    assert!(git(caller, &["diff", "--cached", "--raw"]).is_empty());
    assert!(git(caller, &["diff", "--raw"]).is_empty());
    let mut other_after = snapshot(other);
    other_after.retain(|path, _| !metadata_is_directory || !path.starts_with(".git"));
    assert_eq!(other_after, other_before);
    assert_eq!(fs::read(other_repo.git_dir().join("HEAD")).unwrap(), head_before);
    assert_eq!(fs::read(other_repo.index_path()).unwrap(), index_before);
}

#[test]
fn checkout_target_attached_in_linked_worktree_denies_then_owner_detach_allows_retry() {
    let dir = fixture();
    let parent = tempfile::tempdir().unwrap();
    let other = parent.path().join("other");
    git(dir.path(), &["worktree", "add", "-q", other.to_str().unwrap(), "topic"]);

    occupied_target(dir.path(), &other, REFERENCE);
}

#[test]
fn checkout_target_attached_in_primary_worktree_denies_then_owner_detach_allows_retry() {
    let dir = fixture();
    let parent = tempfile::tempdir().unwrap();
    let caller = parent.path().join("caller");
    git(dir.path(), &["worktree", "add", "-q", caller.to_str().unwrap(), "topic"]);

    occupied_target(&caller, dir.path(), "refs/heads/main");
}

#[test]
fn checkout_target_attached_in_sibling_worktree_denies_then_owner_detach_allows_retry() {
    let dir = fixture();
    let parent = tempfile::tempdir().unwrap();
    let caller = parent.path().join("caller");
    let other = parent.path().join("other");
    git(dir.path(), &["worktree", "add", "-q", "--detach", caller.to_str().unwrap(), "main"]);
    git(dir.path(), &["worktree", "add", "-q", other.to_str().unwrap(), "topic"]);

    occupied_target(&caller, &other, REFERENCE);
}

#[test]
fn checkout_own_attached_branch_in_linked_worktree_preserves_exact_noop() {
    let dir = fixture();
    let parent = tempfile::tempdir().unwrap();
    let linked = parent.path().join("linked");
    git(dir.path(), &["worktree", "add", "-q", linked.to_str().unwrap(), "topic"]);
    fs::write(linked.join("tracked.txt"), b"own staged edit\n").unwrap();
    git(&linked, &["add", "--", "tracked.txt"]);
    fs::write(linked.join("tracked.txt"), b"own unstaged edit\n").unwrap();
    let repo = ProjectRepo::open(&linked).unwrap();
    let primary_before = snapshot(dir.path());
    let linked_before = snapshot(&linked);

    repo.checkout(REFERENCE, actor()).unwrap();

    assert_eq!(
        snapshot(dir.path()),
        primary_before,
        "including per-worktree HEAD, index and reflog bytes"
    );
    assert_eq!(snapshot(&linked), linked_before);
}

#[test]
fn checkout_bare_primary_head_does_not_reserve_branch_for_linked_worktree() {
    let dir = fixture();
    let parent = tempfile::tempdir().unwrap();
    let bare = parent.path().join("bare.git");
    let linked = parent.path().join("linked");
    git(
        dir.path(),
        &["clone", "-q", "--bare", dir.path().to_str().unwrap(), bare.to_str().unwrap()],
    );
    git(&bare, &["symbolic-ref", "HEAD", REFERENCE]);
    git(&bare, &["worktree", "add", "-q", "--detach", linked.to_str().unwrap(), "main"]);
    let repo = ProjectRepo::open(&linked).unwrap();
    let mut before = snapshot(&bare);
    before.retain(|path, _| !path.starts_with("worktrees"));

    switch(&repo, &linked);

    let mut after = snapshot(&bare);
    after.retain(|path, _| !path.starts_with("worktrees"));
    assert_eq!(
        after, before,
        "bare primary HEAD is not a checkout and shared metadata must remain intact"
    );
}
}
