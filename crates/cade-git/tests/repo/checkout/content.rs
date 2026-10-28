//! Successful switching and preservation of local selections.

use super::*;

isolated_tests! {
#[test]
fn checkout_clean_branch_publishes_target_tree_index_and_head_only() {
    let dir = fixture();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let mut before = snapshot(&dir.path().join(".git"));
    before.retain(|path, _| !["HEAD", "index", "logs/HEAD"].iter().any(|allowed| path == Path::new(allowed)));

    switch(&repo, dir.path());

    let mut after = snapshot(&dir.path().join(".git"));
    after.retain(|path, _| !["HEAD", "index", "logs/HEAD"].iter().any(|allowed| path == Path::new(allowed)));
    assert_eq!(
        after, before,
        "objects, config, branch tips/logs and foreign metadata must not change"
    );
    assert!(git(dir.path(), &["diff", "--cached", "--raw"]).is_empty());
    assert!(git(dir.path(), &["diff", "--raw"]).is_empty());
    assert!(
        git(dir.path(), &["ls-files", "--others"]).is_empty(),
        "checkout must not leave private-index artifacts"
    );
}

#[test]
fn checkout_supplied_actor_appends_exact_head_reflog_without_config_identity() {
    let dir = fixture();
    git(dir.path(), &["config", "user.name", ""]);
    git(dir.path(), &["config", "user.email", ""]);
    git(dir.path(), &["config", "user.useConfigOnly", "true"]);
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let old = resolve(dir.path(), "HEAD");
    let target = resolve(dir.path(), REFERENCE);
    let log_path = dir.path().join(".git/logs/HEAD");
    let before = fs::read(&log_path).unwrap();

    switch(&repo, dir.path());

    let after = fs::read(log_path).unwrap();
    assert!(after.starts_with(&before), "existing reflog history must remain byte-identical");
    let appended = std::str::from_utf8(&after[before.len()..]).unwrap();
    assert_eq!(appended.lines().count(), 1);
    assert!(appended.starts_with(&format!(
        "{old} {target} Checkout Actor <checkout@example.invalid> 946688400 +0530\t"
    )));
}

#[test]
fn checkout_unrelated_staged_change_preserves_fresh_index_selection() {
    let dir = fixture();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    fs::write(dir.path().join("keep-stage"), b"user staged bytes\n").unwrap();
    git(dir.path(), &["add", "--", "keep-stage"]);
    let before = git(dir.path(), &["ls-files", "--stage", "--", "keep-stage"]);

    switch(&repo, dir.path());

    assert_eq!(git(dir.path(), &["ls-files", "--stage", "--", "keep-stage"]), before);
    assert_eq!(git(dir.path(), &["show", ":keep-stage"]), b"user staged bytes\n");
    assert_eq!(fs::read(dir.path().join("keep-stage")).unwrap(), b"user staged bytes\n");
}

#[test]
fn checkout_unrelated_unstaged_change_preserves_worktree_bytes() {
    let dir = fixture();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    fs::write(dir.path().join("keep-work"), b"user dirty bytes\n").unwrap();
    let before = git(dir.path(), &["ls-files", "--stage", "--", "keep-work"]);

    switch(&repo, dir.path());

    assert_eq!(fs::read(dir.path().join("keep-work")).unwrap(), b"user dirty bytes\n");
    assert_eq!(git(dir.path(), &["ls-files", "--stage", "--", "keep-work"]), before);
}

#[test]
fn checkout_unrelated_staged_and_unstaged_same_path_preserves_both_versions() {
    let dir = fixture();
    fs::write(dir.path().join("keep-stage"), b"staged version\n").unwrap();
    git(dir.path(), &["add", "--", "keep-stage"]);
    fs::write(dir.path().join("keep-stage"), b"unstaged version\n").unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();

    switch(&repo, dir.path());

    assert_eq!(git(dir.path(), &["show", ":keep-stage"]), b"staged version\n");
    assert_eq!(fs::read(dir.path().join("keep-stage")).unwrap(), b"unstaged version\n");
}

#[test]
fn checkout_unrelated_untracked_file_preserves_native_contents() {
    let dir = fixture();
    fs::write(dir.path().join("keep-untracked"), b"untracked selection\n").unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();

    switch(&repo, dir.path());

    assert_eq!(fs::read(dir.path().join("keep-untracked")).unwrap(), b"untracked selection\n");
    assert!(git(dir.path(), &["ls-files", "--", "keep-untracked"]).is_empty());
}

#[test]
fn checkout_unrelated_ignored_file_preserves_native_contents() {
    let dir = fixture();
    fs::write(dir.path().join("keep-ignored"), b"ignored selection\n").unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();

    switch(&repo, dir.path());

    assert_eq!(fs::read(dir.path().join("keep-ignored")).unwrap(), b"ignored selection\n");
    assert!(git(dir.path(), &["ls-files", "--", "keep-ignored"]).is_empty());
}

#[test]
fn checkout_unrelated_staged_deletion_preserves_absence_and_index() {
    let dir = fixture();
    git(dir.path(), &["rm", "-q", "--", "keep-stage"]);
    let repo = ProjectRepo::open(dir.path()).unwrap();

    switch(&repo, dir.path());

    assert!(!dir.path().join("keep-stage").exists());
    assert!(git(dir.path(), &["ls-files", "--", "keep-stage"]).is_empty());
}

#[test]
fn checkout_unrelated_unstaged_deletion_preserves_absence_and_tracked_entry() {
    let dir = fixture();
    fs::remove_file(dir.path().join("keep-work")).unwrap();
    let before = git(dir.path(), &["ls-files", "--stage", "--", "keep-work"]);
    let repo = ProjectRepo::open(dir.path()).unwrap();

    switch(&repo, dir.path());

    assert!(!dir.path().join("keep-work").exists());
    assert_eq!(git(dir.path(), &["ls-files", "--stage", "--", "keep-work"]), before);
}

#[test]
fn checkout_component_prefix_neighbor_preserves_unrelated_staged_path() {
    let dir = fixture();
    fs::write(dir.path().join("targetdir-neighbor"), b"staged neighbor\n").unwrap();
    git(dir.path(), &["add", "--", "targetdir-neighbor"]);
    let repo = ProjectRepo::open(dir.path()).unwrap();

    switch(&repo, dir.path());

    assert_eq!(git(dir.path(), &["show", ":targetdir-neighbor"]), b"staged neighbor\n");
    assert_eq!(fs::read(dir.path().join("targetdir-neighbor")).unwrap(), b"staged neighbor\n");
}

#[test]
fn checkout_unicode_repository_and_tracked_paths_preserves_native_names() {
    let parent = tempfile::tempdir().unwrap();
    let original = fixture();
    let path = parent.path().join("project space café");
    fs::rename(original.path(), &path).unwrap();
    git(&path, &["checkout", "-q", "topic"]);
    fs::write(path.join("document space 日本語.cade"), b"native target\n").unwrap();
    commit_target_changes(&path);
    let repo = ProjectRepo::open(&path).unwrap();

    switch(&repo, &path);

    assert_eq!(fs::read(path.join("document space 日本語.cade")).unwrap(), b"native target\n");
    assert!(
        git(&path, &["ls-files", "-z"])
            .split(|byte| *byte == 0)
            .any(|name| name == "document space 日本語.cade".as_bytes())
    );
}

// Linux-native byte fixtures: macOS rejected fixture creation with EILSEQ in
// the recorded red run. Assertions remain required on a Linux filesystem.
#[cfg(target_os = "linux")]
#[test]
fn checkout_non_utf8_tracked_path_preserves_exact_native_bytes() {
    use std::os::unix::ffi::OsStringExt;

    let dir = fixture();
    git(dir.path(), &["checkout", "-q", "topic"]);
    let name = std::ffi::OsString::from_vec(b"document-\xff.cade".to_vec());
    fs::write(dir.path().join(&name), b"raw native target\n").unwrap();
    commit_target_changes(dir.path());
    let repo = ProjectRepo::open(dir.path()).unwrap();

    switch(&repo, dir.path());

    assert_eq!(fs::read(dir.path().join(name)).unwrap(), b"raw native target\n");
    assert!(
        git(dir.path(), &["ls-files", "-z"])
            .split(|byte| *byte == 0)
            .any(|name| name == b"document-\xff.cade")
    );
}

#[cfg(target_os = "linux")]
#[test]
fn checkout_non_utf8_repository_path_switches_without_lossy_conversion() {
    use std::os::unix::ffi::OsStringExt;

    let parent = tempfile::tempdir().unwrap();
    let original = fixture();
    let path = parent.path().join(std::ffi::OsString::from_vec(b"project-\xfe".to_vec()));
    fs::rename(original.path(), &path).unwrap();
    let repo = ProjectRepo::open(&path).unwrap();

    switch(&repo, &path);
}

#[cfg(unix)]
#[test]
fn checkout_executable_target_preserves_worktree_and_index_mode() {
    use std::os::unix::fs::PermissionsExt;

    let dir = fixture();
    git(dir.path(), &["config", "core.filemode", "true"]);
    git(dir.path(), &["checkout", "-q", "topic"]);
    fs::set_permissions(dir.path().join("tracked.txt"), fs::Permissions::from_mode(0o755)).unwrap();
    commit_target_changes(dir.path());
    let repo = ProjectRepo::open(dir.path()).unwrap();

    switch(&repo, dir.path());

    assert_eq!(
        fs::metadata(dir.path().join("tracked.txt")).unwrap().permissions().mode() & 0o111,
        0o111
    );
    assert!(git(dir.path(), &["ls-files", "--stage", "--", "tracked.txt"]).starts_with(b"100755 "));
}

#[cfg(unix)]
#[test]
fn checkout_symlink_target_preserves_link_not_dereferenced_contents() {
    let dir = fixture();
    git(dir.path(), &["config", "core.symlinks", "true"]);
    git(dir.path(), &["checkout", "-q", "topic"]);
    std::os::unix::fs::symlink("tracked.txt", dir.path().join("selected-link")).unwrap();
    commit_target_changes(dir.path());
    let repo = ProjectRepo::open(dir.path()).unwrap();

    switch(&repo, dir.path());

    assert!(
        fs::symlink_metadata(dir.path().join("selected-link"))
            .unwrap()
            .file_type()
            .is_symlink()
    );
    assert_eq!(fs::read_link(dir.path().join("selected-link")).unwrap(), Path::new("tracked.txt"));
    assert!(git(dir.path(), &["ls-files", "--stage", "--", "selected-link"]).starts_with(b"120000 "));
    assert_eq!(git(dir.path(), &["show", ":selected-link"]), b"tracked.txt");
}
}
