//! Real index/HEAD/worktree status fixtures using the approved SDK event types.
//! The shared test harness isolates the test process as well as CLI children.

use super::{ProjectRepo, assert_git_error, committed_repo, git, run_isolated, snapshot};
use gix::bstr::ByteSlice;
use gix::diff::index::Change;
use gix::status::{Item, index_worktree::iter::Summary};
use std::fs;
use std::path::Path;

isolated_tests! {
fn status(repo: &ProjectRepo) -> Vec<Item> {
    let result = repo.status();
    assert!(result.is_ok(), "status must succeed: {result:?}");
    result.unwrap()
}

fn assert_worktree_item(items: &[Item], path: &[u8], summary: Summary) {
    assert_eq!(items.len(), 1, "expected exactly one event: {items:?}");
    assert_eq!(items[0].location().as_bytes(), path);
    assert!(
        matches!(&items[0], Item::IndexWorktree(change) if change.summary() == Some(summary)),
        "expected worktree {summary:?}: {items:?}"
    );
}

#[test]
fn status_clean_repository_returns_no_events() {
    let dir = committed_repo();
    let repo = ProjectRepo::open(dir.path()).unwrap();

    assert!(status(&repo).is_empty());
}

#[test]
fn status_modified_file_reports_worktree_modification() {
    let dir = committed_repo();
    fs::write(dir.path().join("tracked.txt"), b"modified and longer\n").unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();

    assert_worktree_item(&status(&repo), b"tracked.txt", Summary::Modified);
}

#[test]
fn status_staged_file_reports_tree_index_modification() {
    let dir = committed_repo();
    fs::write(dir.path().join("tracked.txt"), b"staged and longer\n").unwrap();
    git(dir.path(), &["add", "--", "tracked.txt"]);
    let repo = ProjectRepo::open(dir.path()).unwrap();

    let items = status(&repo);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].location().as_bytes(), b"tracked.txt");
    assert!(matches!(&items[0], Item::TreeIndex(Change::Modification { .. })));
}

#[test]
fn status_staged_and_modified_path_preserves_both_events() {
    let dir = committed_repo();
    fs::write(dir.path().join("tracked.txt"), b"staged\n").unwrap();
    git(dir.path(), &["add", "--", "tracked.txt"]);
    fs::write(dir.path().join("tracked.txt"), b"further worktree edits\n").unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();

    let items = status(&repo);
    assert_eq!(items.len(), 2);
    assert!(items.iter().all(|item| item.location().as_bytes() == b"tracked.txt"));
    assert!(
        items
            .iter()
            .any(|item| matches!(item, Item::TreeIndex(Change::Modification { .. })))
    );
    assert!(
        items
            .iter()
            .any(|item| matches!(item, Item::IndexWorktree(change) if change.summary() == Some(Summary::Modified)))
    );
}

#[test]
fn status_unstaged_deletion_reports_worktree_removal() {
    let dir = committed_repo();
    fs::remove_file(dir.path().join("tracked.txt")).unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();

    assert_worktree_item(&status(&repo), b"tracked.txt", Summary::Removed);
}

#[test]
fn status_staged_deletion_reports_tree_index_deletion() {
    let dir = committed_repo();
    git(dir.path(), &["rm", "-q", "--", "tracked.txt"]);
    let repo = ProjectRepo::open(dir.path()).unwrap();

    let items = status(&repo);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].location().as_bytes(), b"tracked.txt");
    assert!(matches!(&items[0], Item::TreeIndex(Change::Deletion { .. })));
}

#[test]
fn status_untracked_directory_reports_individual_files() {
    let dir = committed_repo();
    fs::create_dir(dir.path().join("drafts")).unwrap();
    fs::write(dir.path().join("drafts/one.txt"), b"one\n").unwrap();
    fs::write(dir.path().join("drafts/two.txt"), b"two\n").unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();

    let items = status(&repo);
    let mut paths: Vec<_> = items.iter().map(|item| item.location().as_bytes()).collect();
    paths.sort_unstable();
    assert_eq!(paths, [b"drafts/one.txt", b"drafts/two.txt"]);
    assert!(
        items
            .iter()
            .all(|item| matches!(item, Item::IndexWorktree(change) if change.summary() == Some(Summary::Added)))
    );
}

#[test]
fn status_config_hides_untracked_still_reports_files_without_config_write() {
    let dir = committed_repo();
    git(dir.path(), &["config", "status.showUntrackedFiles", "no"]);
    fs::write(dir.path().join("draft.txt"), b"draft\n").unwrap();
    let config = fs::read(dir.path().join(".git/config")).unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();

    assert_worktree_item(&status(&repo), b"draft.txt", Summary::Added);
    assert_eq!(fs::read(dir.path().join(".git/config")).unwrap(), config);
}

#[test]
fn status_empty_unborn_repository_returns_no_events() {
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q", "-b", "main"]);
    let repo = ProjectRepo::open(dir.path()).unwrap();

    assert!(status(&repo).is_empty());
    assert!(!dir.path().join(".git/refs/heads/main").exists());
}

#[test]
fn status_unborn_staged_file_reports_tree_index_addition() {
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q", "-b", "main"]);
    fs::write(dir.path().join("first.txt"), b"first\n").unwrap();
    git(dir.path(), &["add", "--", "first.txt"]);
    let repo = ProjectRepo::open(dir.path()).unwrap();

    let items = status(&repo);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].location().as_bytes(), b"first.txt");
    assert!(matches!(&items[0], Item::TreeIndex(Change::Addition { .. })));
    assert!(!dir.path().join(".git/refs/heads/main").exists());
}

#[test]
fn status_index_replaced_between_requests_reads_fresh_entries() {
    let dir = committed_repo();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    assert!(status(&repo).is_empty());
    fs::write(dir.path().join("new.txt"), b"new staged file\n").unwrap();
    git(dir.path(), &["add", "--", "new.txt"]);

    let items = status(&repo);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].location().as_bytes(), b"new.txt");
    assert!(matches!(&items[0], Item::TreeIndex(Change::Addition { .. })));
}

fn conflicted_index(dir: &Path) {
    use gix::index::entry::{Flags, Stage};

    // Construct all three real index stages without invoking merge drivers.
    let base = String::from_utf8(git(dir, &["rev-parse", "HEAD:tracked.txt"])).unwrap();
    fs::write(dir.join("tracked.txt"), b"ours\n").unwrap();
    let ours = String::from_utf8(git(dir, &["hash-object", "-w", "tracked.txt"])).unwrap();
    fs::write(dir.join("tracked.txt"), b"theirs\n").unwrap();
    let theirs = String::from_utf8(git(dir, &["hash-object", "-w", "tracked.txt"])).unwrap();
    fs::write(dir.join("tracked.txt"), b"ours\n").unwrap();
    git(dir, &["add", "--", "tracked.txt"]);
    let mut index = gix::index::File::at(dir.join(".git/index"), gix::hash::Kind::Sha1, false, Default::default()).unwrap();
    let entry = index.entries()[0].clone();
    index.entries_mut()[0].flags = Flags::from_stage(Stage::Base);
    index.entries_mut()[0].id = gix::ObjectId::from_hex(base.trim().as_bytes()).unwrap();
    for (stage, id) in [(Stage::Ours, &ours), (Stage::Theirs, &theirs)] {
        index.dangerously_push_entry(
            entry.stat,
            gix::ObjectId::from_hex(id.trim().as_bytes()).unwrap(),
            Flags::from_stage(stage),
            entry.mode,
            b"tracked.txt".as_bstr(),
        );
    }
    index.sort_entries();
    let _ = index.remove_tree();
    index.write(Default::default()).unwrap();
    assert_eq!(
        String::from_utf8(git(dir, &["ls-files", "--unmerged"])).unwrap(),
        format!(
            "100644 {} 1\ttracked.txt\n100644 {} 2\ttracked.txt\n100644 {} 3\ttracked.txt\n",
            base.trim(),
            ours.trim(),
            theirs.trim()
        )
    );
}

#[test]
fn status_conflicted_index_reports_conflict_without_resolving() {
    let dir = committed_repo();
    conflicted_index(dir.path());
    let stages = String::from_utf8(git(dir.path(), &["ls-files", "--unmerged"])).unwrap();
    assert_eq!(stages.lines().count(), 3);
    let before = snapshot(dir.path());
    let repo = ProjectRepo::open(dir.path()).unwrap();

    let items = status(&repo);
    assert!(items.iter().any(|item| {
        item.location().as_bytes() == b"tracked.txt"
            && matches!(item, Item::IndexWorktree(change) if change.summary() == Some(Summary::Conflict))
    }));
    assert_eq!(snapshot(dir.path()), before);
}

#[test]
fn status_spaces_and_unicode_preserves_relative_path_bytes() {
    let dir = committed_repo();
    let name = "draft 日本語 space.txt";
    fs::write(dir.path().join(name), b"draft\n").unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();

    assert_worktree_item(&status(&repo), name.as_bytes(), Summary::Added);
}

#[test]
fn status_repository_path_with_spaces_and_unicode_returns_relative_paths() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("project 日本語 space");
    fs::create_dir(&root).unwrap();
    git(&root, &["init", "-q", "-b", "main"]);
    git(&root, &["commit", "-q", "--allow-empty", "-m", "Fixture"]);
    fs::write(root.join("draft.txt"), b"draft\n").unwrap();
    let repo = ProjectRepo::open(&root).unwrap();

    assert_worktree_item(&status(&repo), b"draft.txt", Summary::Added);
}

// Linux permits arbitrary non-NUL path bytes; macOS/Windows use the portable
// spaces/Unicode fixture above rather than assuming this filename is supported.
#[cfg(target_os = "linux")]
#[test]
fn status_non_utf8_filename_preserves_native_bytes() {
    use std::os::unix::ffi::OsStrExt;

    let dir = committed_repo();
    let bytes = b"draft-\xff.txt";
    fs::write(dir.path().join(std::ffi::OsStr::from_bytes(bytes)), b"draft\n").unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();

    assert_worktree_item(&status(&repo), bytes, Summary::Added);
}

#[test]
fn status_unchanged_content_with_stale_stat_preserves_all_files() {
    let dir = committed_repo();
    // Replacing the inode guarantees stale stat information without sleeps.
    let path = dir.path().join("tracked.txt");
    let old_inode = dir.path().join(".git/old-tracked");
    fs::rename(&path, &old_inode).unwrap();
    fs::write(&path, b"original\n").unwrap();
    fs::remove_file(old_inode).unwrap();
    let before = snapshot(dir.path());
    let repo = ProjectRepo::open(dir.path()).unwrap();

    assert!(status(&repo).is_empty());
    assert_eq!(snapshot(dir.path()), before);
}

#[test]
fn status_dirty_repository_preserves_index_refs_config_and_worktree() {
    let dir = committed_repo();
    fs::write(dir.path().join("tracked.txt"), b"staged\n").unwrap();
    git(dir.path(), &["add", "--", "tracked.txt"]);
    fs::write(dir.path().join("tracked.txt"), b"unstaged longer\n").unwrap();
    fs::write(dir.path().join("new.txt"), b"untracked\n").unwrap();
    let before = snapshot(dir.path());
    let repo = ProjectRepo::open(dir.path()).unwrap();

    assert!(!status(&repo).is_empty());
    assert_eq!(snapshot(dir.path()), before);
}

#[test]
fn status_bare_repository_returns_git_error_without_mutation() {
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q", "--bare", "-b", "main"]);
    let before = snapshot(dir.path());
    let repo = ProjectRepo::open(dir.path()).unwrap();

    let message = assert_git_error(repo.status()).to_ascii_lowercase();
    assert!(
        message.contains("bare") || message.contains("worktree") || message.contains("working tree"),
        "explain the missing working tree: {message}"
    );
    assert_eq!(snapshot(dir.path()), before);
}

fn assert_corruption_recovers(relative: &str, corrupt: &[u8]) {
    let dir = committed_repo();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let path = dir.path().join(relative);
    let original = fs::read(&path).unwrap();
    fs::write(&path, corrupt).unwrap();
    let before = snapshot(dir.path());

    assert_git_error(repo.status());
    assert_eq!(snapshot(dir.path()), before);

    fs::write(path, original).unwrap();
    assert!(status(&repo).is_empty());
}

#[test]
fn status_corrupt_index_returns_git_error_and_recovers() {
    assert_corruption_recovers(".git/index", b"not an index\n");
}

#[test]
fn status_corrupt_head_returns_git_error_and_recovers() {
    assert_corruption_recovers(".git/HEAD", b"not a ref or object id\n");
}

#[test]
fn status_missing_head_object_returns_git_error_and_recovers() {
    assert_corruption_recovers(".git/refs/heads/main", b"1111111111111111111111111111111111111111\n");
}

#[cfg(unix)]
#[test]
fn status_unreadable_index_returns_git_error_and_recovers() {
    use std::os::unix::fs::PermissionsExt;

    let dir = committed_repo();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let path = dir.path().join(".git/index");
    let permissions = fs::metadata(&path).unwrap().permissions();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o0)).unwrap();
    let denied = fs::read(&path);
    let result = repo.status();
    fs::set_permissions(path, permissions).unwrap();

    assert_eq!(denied.unwrap_err().kind(), std::io::ErrorKind::PermissionDenied);
    assert_git_error(result);
    assert!(status(&repo).is_empty());
}

#[cfg(unix)]
#[test]
fn status_unreadable_tracked_file_returns_git_error_and_recovers() {
    use std::os::unix::fs::PermissionsExt;

    let dir = committed_repo();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let path = dir.path().join("tracked.txt");
    // Keep the length unchanged so a content comparison needs to read the file.
    fs::write(&path, b"modified\n").unwrap();
    let permissions = fs::metadata(&path).unwrap().permissions();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o0)).unwrap();
    let denied = fs::read(&path);
    let result = repo.status();
    fs::set_permissions(&path, permissions).unwrap();

    assert_eq!(denied.unwrap_err().kind(), std::io::ErrorKind::PermissionDenied);
    assert_git_error(result);
    assert_eq!(fs::read(path).unwrap(), b"modified\n");
    assert_worktree_item(&status(&repo), b"tracked.txt", Summary::Modified);
}

// If invoked, the sentinel creates an unmistakable file and exits immediately;
// it cannot hang on the process-filter protocol. Git's shell syntax is also
// available in Git for Windows. No shell is invoked by these tests directly.
fn configure_filter(dir: &Path, kind: &str, used: bool) {
    git(
        dir,
        &[
            "config",
            &format!("filter.sentinel.{kind}"),
            "echo invoked > .git/filter-invoked; exit 1",
        ],
    );
    if used {
        fs::create_dir_all(dir.join(".git/info")).unwrap();
        fs::write(dir.join(".git/info/attributes"), b"tracked.txt filter=sentinel\n").unwrap();
        fs::write(dir.join("tracked.txt"), b"force a filter comparison\n").unwrap();
    }
}

fn assert_filter_denied(kind: &str, used: bool) {
    let dir = committed_repo();
    configure_filter(dir.path(), kind, used);
    let before = snapshot(dir.path());
    let repo = ProjectRepo::open(dir.path()).unwrap();

    let result = repo.status();
    assert!(!dir.path().join(".git/filter-invoked").exists(), "external filter ran");
    assert_eq!(snapshot(dir.path()), before);
    let message = assert_git_error(result);
    assert!(
        message.to_ascii_lowercase().contains("filter"),
        "identify the denied filter: {message}"
    );
}

#[test]
fn status_clean_filter_rejects_before_invocation() {
    assert_filter_denied("clean", true);
}

#[test]
fn status_process_filter_rejects_before_invocation() {
    assert_filter_denied("process", true);
}

#[test]
fn status_unused_executable_filter_rejects_before_invocation() {
    assert_filter_denied("clean", false);
}

#[test]
fn status_smudge_only_filter_never_invokes_external_command() {
    let dir = committed_repo();
    configure_filter(dir.path(), "smudge", true);
    let before = snapshot(dir.path());
    let repo = ProjectRepo::open(dir.path()).unwrap();

    assert_worktree_item(&status(&repo), b"tracked.txt", Summary::Modified);
    assert!(!dir.path().join(".git/filter-invoked").exists());
    assert_eq!(snapshot(dir.path()), before);
}

#[test]
fn status_removed_filter_configuration_allows_explicit_retry() {
    let dir = committed_repo();
    configure_filter(dir.path(), "clean", true);
    assert_git_error(ProjectRepo::open(dir.path()).unwrap().status());
    assert!(!dir.path().join(".git/filter-invoked").exists());
    git(dir.path(), &["config", "--remove-section", "filter.sentinel"]);
    fs::remove_file(dir.path().join(".git/info/attributes")).unwrap();

    assert_worktree_item(&status(&ProjectRepo::open(dir.path()).unwrap()), b"tracked.txt", Summary::Modified);
}

fn nested_repo(dir: &Path) {
    fs::create_dir(dir.join("nested")).unwrap();
    git(&dir.join("nested"), &["init", "-q", "-b", "main"]);
    fs::write(dir.join("nested/tracked.txt"), b"nested\n").unwrap();
    git(&dir.join("nested"), &["add", "--", "tracked.txt"]);
    git(&dir.join("nested"), &["commit", "-q", "-m", "Nested fixture"]);
    configure_filter(&dir.join("nested"), "clean", true);
}

#[test]
fn status_untracked_nested_repository_denies_without_traversal() {
    let dir = committed_repo();
    nested_repo(dir.path());
    let before = snapshot(dir.path());
    let repo = ProjectRepo::open(dir.path()).unwrap();

    let result = repo.status();
    assert!(!dir.path().join("nested/.git/filter-invoked").exists());
    assert_eq!(snapshot(dir.path()), before);
    let message = assert_git_error(result);
    assert!(
        message.to_ascii_lowercase().contains("nested"),
        "identify the nested repository: {message}"
    );
}

#[test]
fn status_tracked_submodule_denies_without_nested_filter_execution() {
    let dir = committed_repo();
    nested_repo(dir.path());
    let id = String::from_utf8(git(&dir.path().join("nested"), &["rev-parse", "HEAD"])).unwrap();
    git(
        dir.path(),
        &["update-index", "--add", "--cacheinfo", &format!("160000,{},nested", id.trim())],
    );
    fs::write(
        dir.path().join(".gitmodules"),
        b"[submodule \"nested\"]\n\tpath = nested\n\turl = ./nested\n",
    )
    .unwrap();
    git(dir.path(), &["add", "--", ".gitmodules"]);
    git(dir.path(), &["commit", "-q", "-m", "Gitlink fixture"]);
    let before = snapshot(dir.path());
    let repo = ProjectRepo::open(dir.path()).unwrap();

    let result = repo.status();
    assert!(!dir.path().join("nested/.git/filter-invoked").exists());
    assert_eq!(snapshot(dir.path()), before);
    let message = assert_git_error(result);
    assert!(
        message.to_ascii_lowercase().contains("submodule"),
        "identify the unsupported submodule: {message}"
    );
}
}
