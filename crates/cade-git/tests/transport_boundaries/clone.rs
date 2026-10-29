use super::*;

isolated_tests! {
#[test]
fn clone_with_transport_advertised_head_checks_out_content() {
    let source = committed_repo();
    git(source.path(), &["branch", "-m", "release"]);
    advance(source.path());
    let root = tempfile::tempdir().unwrap();
    let destination = root.path().join("project with space Ω");
    let (result, _) = with_upload_pack(source.path(), None, |transport| {
        ProjectRepo::clone_with_transport(&destination, &remote(), transport, gix::progress::Discard, &AtomicBool::new(false))
    });
    assert!(result.is_ok(), "clone must receive and check out the advertised branch");
    let repo = result.unwrap();
    assert_eq!(repo.branch().unwrap().as_deref(), Some("release"));
    assert_eq!(
        sdk(&destination).head_id().unwrap().detach(),
        sdk(source.path()).head_id().unwrap().detach()
    );
    assert_eq!(fs::read(destination.join("tracked.txt")).unwrap(), b"remote revision\n");
    assert!(destination.join(".git/index").is_file());
    assert_received(&destination, source.path(), "refs/remotes/origin/release");
    assert!(repo.status().unwrap().is_empty(), "initial index must match both HEAD and worktree");
}

#[test]
fn clone_with_transport_success_persists_only_credential_free_origin() {
    let source = committed_repo();
    let root = tempfile::tempdir().unwrap();
    let destination = root.path().join("clone");
    let (result, _) = with_upload_pack(source.path(), None, |transport| {
        ProjectRepo::clone_with_transport(&destination, &remote(), transport, gix::progress::Discard, &AtomicBool::new(false))
    });
    assert!(result.is_ok(), "clone must complete before checking persisted configuration");
    let repo = sdk(&destination);
    let origin = repo.find_remote("origin").unwrap();
    assert_eq!(origin.url(gix::remote::Direction::Fetch).unwrap().to_bstring(), remote().as_str());
    let config = fs::read_to_string(destination.join(".git/config")).unwrap();
    for forbidden in ["x-access-token", "synthetic-token", "credential", "extraHeader", "proxy"] {
        assert!(
            !config.contains(forbidden),
            "clone must not persist credential or transport overrides"
        );
    }
}

#[test]
fn clone_with_transport_failed_download_retains_initialized_repository() {
    let root = tempfile::tempdir().unwrap();
    let destination = root.path().join("clone");
    let message = assert_git_error(ProjectRepo::clone_with_transport(
        &destination,
        &remote(),
        disconnected(),
        gix::progress::Discard,
        &AtomicBool::new(false),
    ));
    assert!(
        destination.join(".git").is_dir(),
        "download failure must retain the initialized destination"
    );
    assert!(message.contains("initialized"), "failure must identify retained initialized state");
    assert!(
        message.contains("fetch"),
        "failure must describe explicit recovery, not automatic re-clone"
    );
    assert!(ProjectRepo::open(&destination).is_ok());
}

#[test]
fn clone_with_transport_interrupted_advertisement_retains_user_content() {
    let source = committed_repo();
    let root = tempfile::tempdir().unwrap();
    let destination = root.path().join("clone");
    let interrupt = Arc::new(AtomicBool::new(false));
    let (result, _) = with_upload_pack(source.path(), Some(interrupt.clone()), |transport| {
        ProjectRepo::clone_with_transport(&destination, &remote(), transport, gix::progress::Discard, &interrupt)
    });
    assert!(
        interrupt.load(Ordering::SeqCst),
        "the SDK must actually read the local server advertisement"
    );
    let message = assert_git_error(result);
    assert!(
        message.contains("interrupt") || message.contains("cancel"),
        "interruption must be reported"
    );
    assert!(
        destination.join(".git").is_dir(),
        "initialized state must not be recursively removed"
    );
    fs::write(destination.join("owned-after-interruption"), b"keep\n").unwrap();
    let before = snapshot(&destination);
    assert_git_error(ProjectRepo::clone_with_transport(
        &destination,
        &remote(),
        disconnected(),
        gix::progress::Discard,
        &AtomicBool::new(false),
    ));
    assert_eq!(snapshot(&destination), before, "retrying clone must not delete later user content");
}

#[test]
fn clone_with_transport_retained_state_supports_explicit_recovery() {
    let source = committed_repo();
    advance(source.path());
    let root = tempfile::tempdir().unwrap();
    let destination = root.path().join("clone");
    assert_git_error(ProjectRepo::clone_with_transport(
        &destination,
        &remote(),
        disconnected(),
        gix::progress::Discard,
        &AtomicBool::new(false),
    ));
    assert!(destination.join(".git").is_dir(), "failed clone must retain reopenable state");
    fs::write(destination.join("owned-after-failure"), b"keep recovery content\n").unwrap();
    let reopened = ProjectRepo::open(&destination).unwrap();
    let result = fetch(&reopened, source.path());
    assert!(
        result.is_ok(),
        "correcting the source and explicitly fetching must recover the same repository"
    );
    assert_received(&destination, source.path(), "refs/remotes/origin/main");
    let target = sdk(source.path()).head_id().unwrap().detach();
    reopened.create_branch("refs/heads/recovered", target, actor()).unwrap();
    reopened.checkout("refs/heads/recovered", actor()).unwrap();
    assert_eq!(fs::read(destination.join("tracked.txt")).unwrap(), b"remote revision\n");
    assert_eq!(
        fs::read(destination.join("owned-after-failure")).unwrap(),
        b"keep recovery content\n"
    );
}

#[test]
fn clone_with_transport_missing_default_branch_rejects_unrelated_checkout() {
    let source = committed_repo();
    git(source.path(), &["symbolic-ref", "HEAD", "refs/heads/missing"]);
    let root = tempfile::tempdir().unwrap();
    let destination = root.path().join("clone");
    let (result, _) = with_upload_pack(source.path(), None, |transport| {
        ProjectRepo::clone_with_transport(&destination, &remote(), transport, gix::progress::Discard, &AtomicBool::new(false))
    });
    let message = assert_git_error(result);
    assert!(destination.join(".git").is_dir(), "retain downloaded/initialized state");
    assert!(
        !destination.join("tracked.txt").exists(),
        "do not silently check out unrelated main"
    );
    assert!(
        message.contains("HEAD") || message.contains("default branch"),
        "identify the unavailable default branch"
    );
}

#[test]
fn clone_with_transport_empty_remote_never_fabricates_commit() {
    let source = tempfile::tempdir().unwrap();
    git(source.path(), &["init", "-q", "-b", "unborn"]);
    let root = tempfile::tempdir().unwrap();
    let destination = root.path().join("clone");
    let (result, _) = with_upload_pack(source.path(), None, |transport| {
        ProjectRepo::clone_with_transport(&destination, &remote(), transport, gix::progress::Discard, &AtomicBool::new(false))
    });
    assert!(
        destination.join(".git").is_dir(),
        "an empty remote must have explicit retained local state"
    );
    assert!(
        !ProjectRepo::open(&destination).unwrap().has_commits(),
        "never fabricate an initial commit"
    );
    assert!(!destination.join("tracked.txt").exists());
    // The brief allows explicit handling but does not choose empty-clone success
    // versus actionable incompleteness. Do not invent that unresolved decision.
    if let Err(error) = result {
        let message = error.to_string();
        assert!(
            message.contains("empty") || message.contains("unborn"),
            "describe the actual empty remote"
        );
    }
}

#[test]
fn clone_with_transport_gitlink_rejects_checkout_before_writing_entry() {
    let source = committed_repo();
    let target = sdk(source.path()).head_id().unwrap().detach().to_string();
    git(
        source.path(),
        &["update-index", "--add", "--cacheinfo", &format!("160000,{target},nested")],
    );
    git(source.path(), &["commit", "-q", "-m", "Unsupported gitlink"]);
    let root = tempfile::tempdir().unwrap();
    let destination = root.path().join("clone");
    let (result, _) = with_upload_pack(source.path(), None, |transport| {
        ProjectRepo::clone_with_transport(&destination, &remote(), transport, gix::progress::Discard, &AtomicBool::new(false))
    });
    let message = assert_git_error(result);
    assert!(destination.join(".git").is_dir());
    assert!(
        !destination.join("nested").exists(),
        "do not write or initialize the unsupported gitlink"
    );
    assert!(message.contains("gitlink") || message.contains("submodule"));
}

#[test]
fn clone_with_transport_corrupt_server_retains_state_for_corrected_fetch() {
    let source = committed_repo();
    let target = advance(source.path()).to_string();
    let object = source.path().join(".git/objects").join(&target[..2]).join(&target[2..]);
    // Git's loose objects can be read-only. Rename the owned fixture object,
    // rather than mistaking an overwrite-permission failure for server corruption.
    let original = object.with_extension("fixture-backup");
    fs::rename(&object, &original).unwrap();
    fs::write(&object, b"synthetic-provider-canary-not-an-object").unwrap();
    let root = tempfile::tempdir().unwrap();
    let destination = root.path().join("clone");
    let (result, native) = with_upload_pack(source.path(), None, |transport| {
        ProjectRepo::clone_with_transport(&destination, &remote(), transport, gix::progress::Discard, &AtomicBool::new(false))
    });
    let message = assert_git_error(result);
    assert!(
        destination.join(".git").is_dir(),
        "retain actual local state after the native server failure"
    );
    assert!(!native.status.success(), "the fixture must produce a native server failure");
    let native_error = String::from_utf8_lossy(&native.stderr);
    assert!(
        native_error.contains("corrupt") || native_error.contains("inflate") || native_error.contains("bad object"),
        "require the observed native corruption, not a guessed failure category"
    );
    assert!(!message.contains("synthetic-provider-canary"));
    assert!(
        !message.contains(source.path().to_str().unwrap()),
        "provider diagnostics must not escape the boundary"
    );
    fs::remove_file(&object).unwrap();
    fs::rename(original, &object).unwrap();
    let repo = ProjectRepo::open(&destination).unwrap();
    assert!(
        fetch(&repo, source.path()).is_ok(),
        "the corrected source must recover the same retained repository"
    );
    assert_received(&destination, source.path(), "refs/remotes/origin/main");
}
}
