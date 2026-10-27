//! Real branch publication probes. Git CLI only builds or inspects fixtures.

use super::commit::{dirty_selection, initial_repo, resolve, sdk};
use super::objects::{loose_object, malformed_commit, write_commit};
use super::{ProjectRepo, assert_git_error, committed_repo, git, snapshot};
use gix::bstr::ByteSlice;
use std::fs;
use std::path::Path;

#[path = "branches/failures.rs"]
mod failures;
#[path = "branches/validation.rs"]
mod validation;

const REFERENCE: &str = "refs/heads/topic";

fn committer() -> gix::actor::SignatureRef<'static> {
    gix::actor::SignatureRef::from_bytes(b"Supplied Committer <committer@example.invalid> 946688400 +0530").unwrap()
}

fn branch_success(repo: &ProjectRepo, dir: &Path, reference: &str, target: gix::ObjectId) {
    // Unlike commit's protected_state, this permits no new objects or HEAD log
    // changes. Only the requested ref/log and their ancestor directories may vary.
    let metadata = sdk(dir)
        .git_dir()
        .canonicalize()
        .unwrap()
        .strip_prefix(dir.canonicalize().unwrap())
        .unwrap()
        .to_path_buf();
    let ref_path = metadata.join(reference);
    let log_path = metadata.join("logs").join(reference);
    let allowed: std::collections::BTreeSet<_> = [&ref_path, &log_path]
        .into_iter()
        .flat_map(|path| {
            path.ancestors()
                .take_while(|path| *path != metadata.as_path())
                .map(Path::to_path_buf)
        })
        .collect();
    let mut before = snapshot(dir);
    before.retain(|path, _| !allowed.contains(path));

    let result = repo.create_branch(reference, target, committer());

    assert!(result.is_ok(), "valid unused local branch must be created: {result:?}");
    assert_eq!(resolve(dir, reference), target);
    assert_eq!(fs::read(dir.join(ref_path)).unwrap(), format!("{target}\n").as_bytes());
    let mut after = snapshot(dir);
    after.retain(|path, _| !allowed.contains(path));
    assert_eq!(after, before, "only the requested ref/reflog may change");
}

#[test]
fn create_branch_nested_unicode_name_preserves_dirty_worktree_index_head_config_and_other_refs() {
    let dir = committed_repo();
    let target = resolve(dir.path(), "HEAD");
    git(dir.path(), &["update-ref", "refs/heads/other", &target.to_string()]);
    git(dir.path(), &["update-ref", "refs/tags/keep", &target.to_string()]);
    dirty_selection(dir.path());
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let index = dir.path().join(".git/index");
    let before = fs::metadata(&index).unwrap();

    branch_success(&repo, dir.path(), "refs/heads/cade/café", target);

    let after = fs::metadata(index).unwrap();
    assert_eq!(after.modified().unwrap(), before.modified().unwrap());
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        assert_eq!(after.ino(), before.ino(), "the index must not be replaced");
    }
}

#[test]
fn create_branch_supplied_actor_writes_exact_reflog_without_configured_identity() {
    let dir = committed_repo();
    let target = resolve(dir.path(), "HEAD");
    git(dir.path(), &["config", "user.name", ""]);
    git(dir.path(), &["config", "user.email", ""]);
    git(dir.path(), &["config", "user.useConfigOnly", "true"]);
    let repo = ProjectRepo::open(dir.path()).unwrap();

    branch_success(&repo, dir.path(), REFERENCE, target);

    let log = fs::read_to_string(dir.path().join(".git/logs").join(REFERENCE)).unwrap();
    assert_eq!(log.lines().count(), 1);
    assert!(
        log.starts_with(&format!(
            "0000000000000000000000000000000000000000 {target} Supplied Committer <committer@example.invalid> 946688400 +0530\t"
        )),
        "the explicit actor and timestamp must be preserved: {log}"
    );
}

#[test]
fn create_branch_detached_head_preserves_attachment_and_content() {
    let dir = committed_repo();
    let target = resolve(dir.path(), "HEAD");
    git(dir.path(), &["checkout", "-q", "--detach"]);
    let repo = ProjectRepo::open(dir.path()).unwrap();

    branch_success(&repo, dir.path(), REFERENCE, target);

    assert_eq!(repo.branch().unwrap(), None);
    assert_eq!(resolve(dir.path(), "HEAD"), target);
}

#[test]
fn create_branch_unborn_head_with_available_target_preserves_missing_index_and_attachment() {
    let dir = initial_repo();
    let target = write_commit(dir.path(), &[], "Available but unreferenced");
    let repo = ProjectRepo::open(dir.path()).unwrap();

    branch_success(&repo, dir.path(), REFERENCE, target);

    assert!(!repo.has_commits());
    assert_eq!(repo.branch().unwrap().as_deref(), Some("main"));
    assert!(!dir.path().join(".git/index").exists());
}

#[test]
fn create_branch_bare_repository_with_available_target_preserves_unborn_head() {
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q", "--bare", "-b", "main"]);
    let target = write_commit(dir.path(), &[], "Available in bare repo");
    let repo = ProjectRepo::open(dir.path()).unwrap();

    branch_success(&repo, dir.path(), REFERENCE, target);

    assert!(repo.workdir().is_none());
    assert!(!repo.has_commits());
    assert!(!dir.path().join("index").exists());
}

#[test]
fn create_branch_space_and_unicode_repository_path_preserves_native_files() {
    let parent = tempfile::tempdir().unwrap();
    let fixture = committed_repo();
    let path = parent.path().join("project space café");
    fs::rename(fixture.path(), &path).unwrap();
    let native_file = path.join("document space 日本語.cade");
    fs::write(&native_file, b"preserve native path contents\n").unwrap();
    let target = resolve(&path, "HEAD");
    let repo = ProjectRepo::open(&path).unwrap();

    branch_success(&repo, &path, REFERENCE, target);

    assert_eq!(fs::read(native_file).unwrap(), b"preserve native path contents\n");
}

// Match status's Linux-only raw-byte fixture: macOS rejects this directory
// rename with EILSEQ. Spaces/Unicode coverage above remains platform-independent.
#[cfg(target_os = "linux")]
#[test]
fn create_branch_non_utf8_repository_and_index_paths_preserves_actual_bytes() {
    use std::os::unix::ffi::OsStringExt;

    let parent = tempfile::tempdir().unwrap();
    let fixture = committed_repo();
    let path = parent.path().join(std::ffi::OsString::from_vec(b"project-\xff".to_vec()));
    fs::rename(fixture.path(), &path).unwrap();
    let native_file = path.join(std::ffi::OsString::from_vec(b"document-\xfe.cade".to_vec()));
    fs::write(&native_file, b"staged native bytes\n").unwrap();
    git(&path, &["add", "-A"]);
    fs::write(&native_file, b"dirty native bytes\n").unwrap();
    let target = resolve(&path, "HEAD");
    let repo = ProjectRepo::open(&path).unwrap();

    branch_success(&repo, &path, REFERENCE, target);

    assert!(
        git(&path, &["ls-files", "-z"])
            .windows(b"document-\xfe.cade\0".len())
            .any(|p| p == b"document-\xfe.cade\0")
    );
    assert_eq!(fs::read(native_file).unwrap(), b"dirty native bytes\n");
}
