//! Actual SDK veto, object-read and OS failures caused by owned-fixture faults.
//! No seam fabricates an error or replaces checkout/publication with a mock.

use super::*;

#[test]
fn checkout_native_veto_with_false_ok_denies_publication_then_corrected_retry_succeeds() {
    let dir = fixture();
    dirty_selection(dir.path());
    fs::write(dir.path().join("tracked.txt"), b"owned overlapping edit\n").unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let before = snapshot(dir.path());
    let phases = Rc::new(RefCell::new(Vec::new()));
    let recorded = phases.clone();
    let scope = observer::install(move |phase| recorded.borrow_mut().push(phase));

    let error = assert_git_error(repo.checkout(REFERENCE, actor()));
    drop(scope);

    assert!(
        phases.borrow().contains(&Phase::SdkReturned {
            vetoed: true,
            succeeded: true,
        }),
        "pinned native/binding false-Ok must actually be observed, not injected: {:?}",
        phases.borrow()
    );
    assert!(!phases.borrow().contains(&Phase::WorktreeUpdated));
    assert!(!phases.borrow().contains(&Phase::HeadPublished));
    assert!(error.contains("conflict") && error.contains("not published"), "{error}");
    assert_eq!(snapshot(dir.path()), before);
    assert_unlocked(dir.path());
    // Archive the owner's edit before explicitly restoring just this fixture
    // path from the existing index. Production does neither step.
    let archive = tempfile::tempdir().unwrap();
    fs::rename(dir.path().join("tracked.txt"), archive.path().join("tracked.txt")).unwrap();
    git(dir.path(), &["checkout-index", "--", "tracked.txt"]);
    repo.checkout(REFERENCE, actor()).unwrap();
    assert_eq!(fs::read(archive.path().join("tracked.txt")).unwrap(), b"owned overlapping edit\n");
    assert_target(dir.path());
    assert_selection(dir.path());
}

#[test]
fn checkout_late_worktree_blob_read_failure_reports_partial_state_then_corrected_retry_succeeds() {
    let dir = fixture();
    dirty_selection(dir.path());
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let metadata = snapshot(&dir.path().join(".git"));
    let blob = git(dir.path(), &["rev-parse", "topic:targetdir/child"]);
    let blob = std::str::from_utf8(&blob).unwrap().trim();
    let object = Path::new("objects").join(&blob[..2]).join(&blob[2..]);
    let loose_blob = dir.path().join(".git").join(&object);
    let original_blob = fs::read(&loose_blob).unwrap();
    let archive = tempfile::tempdir().unwrap();
    let saved_blob = archive.path().join("late-blob");
    let backup = saved_blob.clone();
    let injected = Rc::new(RefCell::new(false));
    let performed = injected.clone();
    let phases = Rc::new(RefCell::new(Vec::new()));
    let recorded = phases.clone();
    let path = dir.path().to_path_buf();
    let scope = observer::install(move |phase| {
        recorded.borrow_mut().push(phase);
        if phase == Phase::SdkProgress && !*performed.borrow() && path.join("a-early").exists() {
            // libgit2 1.9.7 checkout_create_the_new reports progress after each
            // blob write; checkout_write_content looks up the next blob before
            // opening its output. Gix validation has finished, but its cache is
            // independent of the native repository. Archive a later loose blob
            // so the native lookup, not a fabricated Result, must fail.
            assert_eq!(fs::read(path.join("a-early")).unwrap(), b"first changed path\n");
            assert!(
                !path.join("targetdir/child").exists(),
                "fault must precede the selected late output"
            );
            fs::rename(&loose_blob, &backup).unwrap();
            assert!(!loose_blob.exists());
            // Retain the original late-obstruction preservation canary. It is
            // not our evidence of an OS write failure (chmod is not one either).
            fs::create_dir(path.join("z-new")).unwrap();
            fs::write(path.join("z-new/user-entry"), b"preserve late obstruction\n").unwrap();
            *performed.borrow_mut() = true;
        }
    });

    let error = assert_git_error(repo.checkout(REFERENCE, actor()));
    drop(scope);

    assert!(
        *injected.borrow(),
        "owned obstruction must be installed after an actual worktree write"
    );
    assert!(
        phases.borrow().contains(&Phase::SdkReturned {
            vetoed: false,
            succeeded: false,
        }),
        "must observe SDK failure, not a preflight conflict or a fabricated Result"
    );
    assert!(
        phases.borrow().contains(&Phase::SdkFailed {
            code: git2::ErrorCode::NotFound,
            class: git2::ErrorClass::Odb,
        }),
        "must observe the actual native object-read failure, not mislabel it as an OS write error: {:?}",
        phases.borrow()
    );
    assert!(!phases.borrow().contains(&Phase::WorktreeUpdated));
    assert!(!phases.borrow().contains(&Phase::HeadPublished));
    for category in [
        "partial state",
        "HEAD has not been published",
        "real index was not published",
        "explicitly",
        "retry",
    ] {
        assert!(error.contains(category), "{error}");
    }
    let mut missing_blob_metadata = metadata.clone();
    assert!(
        missing_blob_metadata.remove(&object).is_some(),
        "only the fixture-owned blob was removed"
    );
    assert_eq!(snapshot(&dir.path().join(".git")), missing_blob_metadata);
    assert_eq!(fs::read(&saved_blob).unwrap(), original_blob);
    assert_eq!(fs::read(dir.path().join("a-early")).unwrap(), b"first changed path\n");
    assert_eq!(
        fs::read(dir.path().join("z-new/user-entry")).unwrap(),
        b"preserve late obstruction\n"
    );
    assert_selection(dir.path());
    assert_unlocked(dir.path());

    // An unchanged obstructed request may not silently repair or discard it.
    let partial = snapshot(dir.path());
    assert!(repo.checkout(REFERENCE, actor()).is_err());
    assert_eq!(snapshot(dir.path()), partial);

    // Explicit caller correction: restore the exact archived object, archive
    // this fixture's checkout outputs and injected obstruction, then restore
    // baseline paths from the OLD index.
    // Unrelated tracked/staged/unstaged/untracked/ignored edits remain in place.
    fs::rename(&saved_blob, dir.path().join(".git").join(&object)).unwrap();
    assert_eq!(snapshot(&dir.path().join(".git")), metadata);
    for name in ["a-early", "tracked.txt", "removed.txt", "targetdir", "z-ignored", "z-new"] {
        let path = dir.path().join(name);
        if path.exists() {
            fs::rename(path, archive.path().join(name)).unwrap();
        }
    }
    git(dir.path(), &["checkout-index", "--", "tracked.txt", "removed.txt"]);
    repo.checkout(REFERENCE, actor()).unwrap();
    assert_eq!(fs::read(archive.path().join("a-early")).unwrap(), b"first changed path\n");
    assert_eq!(
        fs::read(archive.path().join("z-new/user-entry")).unwrap(),
        b"preserve late obstruction\n"
    );
    assert_target(dir.path());
    assert_selection(dir.path());
    assert_unlocked(dir.path());
}

#[test]
fn checkout_late_index_obstruction_reports_published_head_then_explicit_index_correction_allows_retry() {
    let dir = fixture();
    dirty_selection(dir.path());
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let original_index = fs::read(dir.path().join(".git/index")).unwrap();
    let head_log = fs::read(dir.path().join(".git/logs/HEAD")).unwrap();
    let mut metadata = snapshot(&dir.path().join(".git"));
    metadata.retain(|path, _| !["HEAD", "index", "logs/HEAD"].iter().any(|allowed| path == Path::new(allowed)));
    let archive = tempfile::tempdir().unwrap();
    let saved_index = archive.path().join("original-index");
    let prepared = Rc::new(RefCell::new(None));
    let captured = prepared.clone();
    let path = dir.path().to_path_buf();
    let backup = saved_index.clone();
    let scope = observer::install(move |phase| {
        if phase != Phase::HeadPublished {
            return;
        }
        assert!(captured.borrow().is_none(), "publication must not be automatically retried");
        *captured.borrow_mut() = Some(fs::read(path.join(".git/index.lock")).unwrap());
        fs::rename(path.join(".git/index"), &backup).unwrap();
        fs::create_dir(path.join(".git/index")).unwrap();
        fs::write(path.join(".git/index/user-entry"), b"preserve index obstruction\n").unwrap();
    });

    let error = assert_git_error(repo.checkout(REFERENCE, actor()));
    drop(scope);

    for category in [
        "partial state",
        "HEAD was published",
        "final index publication failed",
        "explicitly retry",
    ] {
        assert!(error.contains(category), "{error}");
    }
    assert!(
        prepared.borrow().is_some(),
        "fault must occur after real HEAD publication and index preparation"
    );
    assert_eq!(fs::read(&saved_index).unwrap(), original_index);
    assert_eq!(fs::read(dir.path().join(".git/HEAD")).unwrap(), b"ref: refs/heads/topic\n");
    assert_eq!(fs::read(dir.path().join("tracked.txt")).unwrap(), b"topic tracked bytes\n");
    assert_eq!(fs::read(dir.path().join("a-early")).unwrap(), b"first changed path\n");
    assert_eq!(
        fs::read(dir.path().join(".git/index/user-entry")).unwrap(),
        b"preserve index obstruction\n"
    );
    let mut after = snapshot(&dir.path().join(".git"));
    after.retain(|path, _| path != Path::new("HEAD") && !path.starts_with("index") && path != Path::new("logs/HEAD"));
    assert_eq!(
        after, metadata,
        "objects, configuration, other refs/reflogs and metadata remain intact"
    );
    let published_log = fs::read(dir.path().join(".git/logs/HEAD")).unwrap();
    assert!(
        published_log.starts_with(&head_log),
        "HEAD publication must preserve earlier reflog bytes"
    );
    assert_ne!(published_log, head_log, "late failure really follows HEAD/reflog publication");
    assert_worktree_selection(dir.path());
    assert_unlocked(dir.path());

    let partial = snapshot(dir.path());
    assert!(repo.checkout(REFERENCE, actor()).is_err());
    assert_eq!(snapshot(dir.path()), partial, "unchanged retry must not repair partial state");

    // Explicit owner correction, not rollback: archive the obstruction, then
    // install the exact SDK-produced bytes captured before the failed rename.
    // Do not reconstruct a tree index and thereby lose unrelated staging.
    fs::rename(dir.path().join(".git/index"), archive.path().join("obstruction")).unwrap();
    fs::write(dir.path().join(".git/index"), prepared.borrow().as_ref().unwrap()).unwrap();
    let corrected = snapshot(dir.path());
    repo.checkout(REFERENCE, actor()).unwrap();
    assert_eq!(
        snapshot(dir.path()),
        corrected,
        "own already-current branch is a no-op after explicit correction"
    );
    assert_eq!(fs::read(saved_index).unwrap(), original_index);
    assert_eq!(
        fs::read(archive.path().join("obstruction/user-entry")).unwrap(),
        b"preserve index obstruction\n"
    );
    assert_target(dir.path());
    assert_selection(dir.path());
    assert_unlocked(dir.path());
}
