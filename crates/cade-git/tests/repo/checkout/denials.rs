//! Known collisions must veto before even the first alphabetic target write.

use super::*;

isolated_tests! {
#[test]
fn checkout_overlapping_unstaged_edit_denies_before_any_write() {
    let dir = fixture();
    fs::write(dir.path().join("tracked.txt"), b"user dirty selection\n").unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();

    deny_default(&repo, dir.path(), &["conflict", "overwrite", "tracked.txt"]);
}

#[test]
fn checkout_overlapping_staged_edit_denies_before_any_write() {
    let dir = fixture();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    fs::write(dir.path().join("tracked.txt"), b"user staged selection\n").unwrap();
    git(dir.path(), &["add", "--", "tracked.txt"]);

    deny_default(&repo, dir.path(), &["staged"]);
}

#[test]
fn checkout_overlapping_staged_target_bytes_denies_without_consuming_selection() {
    let dir = fixture();
    fs::write(dir.path().join("tracked.txt"), b"topic tracked bytes\n").unwrap();
    git(dir.path(), &["add", "--", "tracked.txt"]);
    let repo = ProjectRepo::open(dir.path()).unwrap();

    deny_default(&repo, dir.path(), &["staged"]);
}

#[test]
fn checkout_overlapping_staged_deletion_denies_before_any_write() {
    let dir = fixture();
    git(dir.path(), &["rm", "-q", "--", "tracked.txt"]);
    let repo = ProjectRepo::open(dir.path()).unwrap();

    deny_default(&repo, dir.path(), &["staged"]);
}

#[test]
fn checkout_overlapping_unstaged_deletion_denies_before_recreating_user_file() {
    let dir = fixture();
    fs::remove_file(dir.path().join("tracked.txt")).unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();

    deny_default(&repo, dir.path(), &["conflict", "overwrite", "tracked.txt"]);
}

#[test]
fn checkout_removed_target_with_local_edit_denies_before_deleting_user_bytes() {
    let dir = fixture();
    fs::write(dir.path().join("removed.txt"), b"preserve edited removal\n").unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();

    deny_default(&repo, dir.path(), &["conflict", "overwrite", "removed.txt"]);
}

#[test]
fn checkout_overlapping_untracked_file_denies_before_any_write() {
    let dir = fixture();
    fs::write(dir.path().join("z-new"), b"user untracked selection\n").unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();

    deny_default(&repo, dir.path(), &["conflict", "overwrite", "untracked", "z-new"]);
}

#[test]
fn checkout_overlapping_ignored_file_denies_before_any_write() {
    let dir = fixture();
    fs::write(dir.path().join("z-ignored"), b"user ignored selection\n").unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();

    deny_default(&repo, dir.path(), &["conflict", "overwrite", "ignored"]);
}

#[test]
fn checkout_file_obstructing_target_directory_denies_before_any_write() {
    let dir = fixture();
    fs::write(dir.path().join("targetdir"), b"user structural obstruction\n").unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();

    deny_default(&repo, dir.path(), &["conflict", "overwrite", "obstruct", "targetdir"]);
}

#[test]
fn checkout_directory_obstructing_target_file_denies_before_any_write() {
    let dir = fixture();
    fs::create_dir(dir.path().join("z-new")).unwrap();
    fs::write(dir.path().join("z-new/user-child"), b"preserve directory child\n").unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();

    deny_default(&repo, dir.path(), &["conflict", "overwrite", "obstruct", "z-new"]);
}

#[test]
fn checkout_staged_ancestor_obstructing_target_child_denies_before_any_write() {
    let dir = fixture();
    fs::write(dir.path().join("targetdir"), b"staged ancestor\n").unwrap();
    git(dir.path(), &["add", "--", "targetdir"]);
    let repo = ProjectRepo::open(dir.path()).unwrap();

    deny_default(&repo, dir.path(), &["staged"]);
}

#[test]
fn checkout_staged_child_obstructing_target_ancestor_denies_before_any_write() {
    let dir = fixture();
    fs::create_dir(dir.path().join("z-new")).unwrap();
    fs::write(dir.path().join("z-new/child"), b"staged child\n").unwrap();
    git(dir.path(), &["add", "--", "z-new/child"]);
    let repo = ProjectRepo::open(dir.path()).unwrap();

    deny_default(&repo, dir.path(), &["staged"]);
}

#[cfg(unix)]
#[test]
fn checkout_symlink_obstructing_target_directory_denies_without_traversal() {
    let dir = fixture();
    let outside = tempfile::tempdir().unwrap();
    fs::write(outside.path().join("child"), b"outside canary\n").unwrap();
    std::os::unix::fs::symlink(outside.path(), dir.path().join("targetdir")).unwrap();
    let external_before = snapshot(outside.path());
    let repo = ProjectRepo::open(dir.path()).unwrap();

    deny_default(&repo, dir.path(), &["conflict", "overwrite", "obstruct", "targetdir"]);

    assert_eq!(snapshot(outside.path()), external_before);
}

fn filter_denied(kind: &str, used: bool) {
    let dir = fixture();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    // Install after opening to require fresh config. Any accidental invocation
    // can only write the canary inside this owned synthetic repository.
    git(
        dir.path(),
        &["config", &format!("filter.blocked.{kind}"), "printf executed > filter-ran"],
    );
    if used {
        fs::write(dir.path().join(".gitattributes"), b"* filter=blocked\n").unwrap();
    }

    deny_default(&repo, dir.path(), &["filter"]);

    assert!(!dir.path().join("filter-ran").exists());
    git(dir.path(), &["config", "--remove-section", "filter.blocked"]);
    if used {
        fs::remove_file(dir.path().join(".gitattributes")).unwrap();
    }
    switch(&repo, dir.path());
}

#[test]
fn checkout_clean_filter_denies_without_execution_then_config_correction_succeeds() {
    filter_denied("clean", true);
}

#[test]
fn checkout_smudge_filter_denies_without_execution_then_config_correction_succeeds() {
    filter_denied("smudge", true);
}

#[test]
fn checkout_process_filter_denies_without_execution_then_config_correction_succeeds() {
    filter_denied("process", true);
}

#[test]
fn checkout_unused_executable_filter_denies_before_any_write() {
    filter_denied("smudge", false);
}

#[test]
fn checkout_untracked_nested_repository_denies_without_traversal() {
    let dir = fixture();
    let nested = dir.path().join("nested");
    fs::create_dir(&nested).unwrap();
    git(&nested, &["init", "-q", "-b", "main"]);
    fs::write(nested.join("user-data"), b"nested user selection\n").unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();

    deny_default(&repo, dir.path(), &["nested", "repository traversal"]);
}

#[test]
fn checkout_nested_bare_repository_denies_without_traversal() {
    let dir = fixture();
    let nested = dir.path().join("nested");
    fs::create_dir(&nested).unwrap();
    git(&nested, &["init", "-q", "--bare", "-b", "main"]);
    let repo = ProjectRepo::open(dir.path()).unwrap();

    deny_default(&repo, dir.path(), &["nested", "repository traversal"]);
}

#[test]
fn checkout_target_gitlink_denies_before_any_write() {
    let dir = fixture();
    git(dir.path(), &["checkout", "-q", "topic"]);
    let target = resolve(dir.path(), "HEAD").to_string();
    git(
        dir.path(),
        &["update-index", "--add", "--cacheinfo", &format!("160000,{target},module")],
    );
    git(dir.path(), &["commit", "-q", "-m", "Target gitlink"]);
    git(dir.path(), &["checkout", "-q", "main"]);
    let repo = ProjectRepo::open(dir.path()).unwrap();

    deny_default(&repo, dir.path(), &["gitlink", "submodule"]);
}

#[test]
fn checkout_index_gitlink_denies_before_any_write() {
    let dir = fixture();
    let target = resolve(dir.path(), "HEAD").to_string();
    git(
        dir.path(),
        &["update-index", "--add", "--cacheinfo", &format!("160000,{target},module")],
    );
    let repo = ProjectRepo::open(dir.path()).unwrap();

    deny_default(&repo, dir.path(), &["gitlink", "submodule"]);
}

#[test]
fn checkout_baseline_gitlink_denies_before_any_write() {
    let dir = fixture();
    let target = resolve(dir.path(), "HEAD").to_string();
    git(
        dir.path(),
        &["update-index", "--add", "--cacheinfo", &format!("160000,{target},module")],
    );
    git(dir.path(), &["commit", "-q", "-m", "Baseline gitlink"]);
    // Remove only the index gitlink: baseline inspection must still reject it.
    git(dir.path(), &["update-index", "--force-remove", "--", "module"]);
    let repo = ProjectRepo::open(dir.path()).unwrap();

    deny_default(&repo, dir.path(), &["gitlink", "submodule"]);
}
}
