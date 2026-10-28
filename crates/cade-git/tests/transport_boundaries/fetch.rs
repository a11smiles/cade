use super::*;

isolated_tests! {
#[test]
fn fetch_with_transport_new_remote_commit_preserves_local_selection() {
    let source = committed_repo();
    let local = committed_repo();
    // Both independent fixtures have the same deterministic baseline commit.
    assert_eq!(
        sdk(source.path()).head_id().unwrap().detach(),
        sdk(local.path()).head_id().unwrap().detach()
    );
    let target = advance(source.path());
    git(local.path(), &["commit", "-q", "--allow-empty", "-m", "Local-only commit"]);
    fs::write(local.path().join("tracked.txt"), b"staged selection\n").unwrap();
    git(local.path(), &["add", "--", "tracked.txt"]);
    fs::write(local.path().join("tracked.txt"), b"unstaged selection\n").unwrap();
    fs::write(local.path().join("untracked"), b"owned bytes\n").unwrap();
    let repo = ProjectRepo::open(local.path()).unwrap();
    assert!(
        sdk(local.path()).try_find_object(target).unwrap().is_none(),
        "fixture needs a genuinely new object"
    );
    let before = selection(local.path());
    let outcome = fetch(&repo, source.path());
    assert!(outcome.is_ok(), "fetch must actually receive remote objects");
    let outcome = outcome.unwrap();
    let gix::remote::fetch::Status::Change { update_refs, .. } = &outcome.status else {
        panic!("success must include a received pack");
    };
    assert!(!update_refs.edits.is_empty(), "success must include publication, not just download");
    assert!(
        update_refs.updates.iter().all(|update| matches!(
            update.mode,
            gix::remote::fetch::refs::update::Mode::New
                | gix::remote::fetch::refs::update::Mode::FastForward
                | gix::remote::fetch::refs::update::Mode::NoChangeNeeded
                | gix::remote::fetch::refs::update::Mode::Forced
        )),
        "a top-level successful result must not hide rejected update mappings"
    );
    assert_eq!(outcome.handshake.server_protocol_version, Protocol::V2);
    assert_received(local.path(), source.path(), "refs/remotes/origin/main");
    assert_eq!(
        selection(local.path()),
        before,
        "fetch must preserve local commits, HEAD, index, configuration and all working bytes"
    );
}

#[test]
fn fetch_with_transport_configured_branch_mapping_updates_only_tracking_ref() {
    let source = committed_repo();
    let local = committed_repo();
    advance(source.path());
    git(
        local.path(),
        &[
            "config",
            "remote.origin.url",
            "https://other.example.invalid/unselected/project.git",
        ],
    );
    git(local.path(), &["config", "remote.origin.fetch", "+refs/heads/*:refs/heads/*"]);
    git(local.path(), &["config", "url.ext::synthetic-helper.insteadOf", remote().as_str()]);
    let repo = ProjectRepo::open(local.path()).unwrap();
    let before = selection(local.path());
    assert!(
        fetch(&repo, source.path()).is_ok(),
        "explicit fetch must ignore configured mappings and rewrites"
    );
    assert_received(local.path(), source.path(), "refs/remotes/origin/main");
    assert_eq!(selection(local.path()), before);
}

#[test]
fn fetch_with_transport_symbolic_tracking_target_preserves_local_branch() {
    let source = committed_repo();
    let local = committed_repo();
    advance(source.path());
    git(local.path(), &["symbolic-ref", "refs/remotes/origin/main", "refs/heads/main"]);
    let repo = ProjectRepo::open(local.path()).unwrap();
    let before = selection(local.path());
    let symbolic = fs::read(local.path().join(".git/refs/remotes/origin/main")).unwrap();
    let message = assert_git_error(fetch(&repo, source.path()));
    assert!(
        message.contains("symbolic"),
        "reject a symbolic destination rather than dereference it"
    );
    assert_eq!(selection(local.path()), before);
    assert_eq!(fs::read(local.path().join(".git/refs/remotes/origin/main")).unwrap(), symbolic);
}

#[test]
fn fetch_with_transport_tracking_lock_supports_corrected_retry() {
    let source = committed_repo();
    let local = committed_repo();
    advance(source.path());
    let tracking = local.path().join(".git/refs/remotes/origin");
    fs::create_dir_all(&tracking).unwrap();
    fs::write(tracking.join("main.lock"), b"other writer\n").unwrap();
    let repo = ProjectRepo::open(local.path()).unwrap();
    let before = selection(local.path());
    let message = assert_git_error(fetch(&repo, source.path()));
    assert!(
        message.contains("lock") || message.contains("publish"),
        "report the observed publication obstruction"
    );
    assert_eq!(fs::read(tracking.join("main.lock")).unwrap(), b"other writer\n");
    assert!(!tracking.join("main").exists());
    assert_eq!(selection(local.path()), before);
    fs::remove_file(tracking.join("main.lock")).unwrap();
    assert!(
        fetch(&repo, source.path()).is_ok(),
        "only the explicitly corrected retry may publish"
    );
    assert_received(local.path(), source.path(), "refs/remotes/origin/main");
    assert_eq!(selection(local.path()), before);
}

#[test]
fn fetch_with_transport_disconnected_server_preserves_selection_for_retry() {
    let source = committed_repo();
    let local = committed_repo();
    advance(source.path());
    let repo = ProjectRepo::open(local.path()).unwrap();
    let before = selection(local.path());
    assert_git_error(repo.fetch_with_transport(&remote(), disconnected(), gix::progress::Discard, &AtomicBool::new(false)));
    assert_eq!(selection(local.path()), before);
    assert!(
        fetch(&repo, source.path()).is_ok(),
        "a later explicit request must not inherit failed connection state"
    );
    assert_received(local.path(), source.path(), "refs/remotes/origin/main");
    assert_eq!(selection(local.path()), before);
}

#[test]
fn fetch_with_transport_interrupted_advertisement_preserves_selection_for_retry() {
    let source = committed_repo();
    let local = committed_repo();
    advance(source.path());
    let repo = ProjectRepo::open(local.path()).unwrap();
    let before = selection(local.path());
    let interrupt = Arc::new(AtomicBool::new(false));
    let (result, _) = with_upload_pack(source.path(), Some(interrupt.clone()), |transport| {
        repo.fetch_with_transport(&remote(), transport, gix::progress::Discard, &interrupt)
    });
    assert!(
        interrupt.load(Ordering::SeqCst),
        "interruption must occur after an actual protocol read"
    );
    let message = assert_git_error(result);
    assert!(message.contains("interrupt") || message.contains("cancel"));
    assert_eq!(selection(local.path()), before);
    interrupt.store(false, Ordering::SeqCst);
    let (result, _) = with_upload_pack(source.path(), None, |transport| {
        repo.fetch_with_transport(&remote(), transport, gix::progress::Discard, &interrupt)
    });
    assert!(result.is_ok(), "the caller's cleared interrupt permits a fresh explicit retry");
    assert_received(local.path(), source.path(), "refs/remotes/origin/main");
    assert_eq!(selection(local.path()), before);
}

#[test]
fn fetch_with_transport_supplied_progress_receives_sdk_messages() {
    let source = committed_repo();
    let local = committed_repo();
    advance(source.path());
    let repo = ProjectRepo::open(local.path()).unwrap();
    let progress = gix::progress::prodash::tree::Root::new();
    let (result, _) = with_upload_pack(source.path(), None, |transport| {
        repo.fetch_with_transport(&remote(), transport, progress.add_child("synthetic fetch"), &AtomicBool::new(false))
    });
    assert!(result.is_ok(), "progress observation requires actual object transfer");
    let mut messages = Vec::new();
    progress.copy_messages(&mut messages);
    assert!(
        !messages.is_empty(),
        "SDK stage progress must reach the supplied sink instead of Discard"
    );
    assert_received(local.path(), source.path(), "refs/remotes/origin/main");
}
}
