//! Persistent regressions from independently executed transport-boundary probes.

use super::*;

isolated_tests! {
#[test]
fn clone_with_connection_late_file_collision_retains_bytes_for_explicit_recovery() {
    let source = committed_repo();
    advance(source.path());
    let root = tempfile::tempdir().unwrap();
    let destination = root.path().join("clone");
    let (result, _) = with_upload_pack(source.path(), None, |transport| {
        ProjectRepo::clone_with_connection(&destination, &remote(), gix::progress::Discard, &AtomicBool::new(false), |remote| {
            // The external writer arrives after clone atomically claims and
            // initializes the destination, not during occupied-path preflight.
            fs::write(destination.join("tracked.txt"), b"QA599 owned collision\n").unwrap();
            fs::write(destination.join("keep-me"), b"QA599 owned sentinel\n").unwrap();
            Ok(remote
                .to_connection_with_transport(transport)
                .with_transport_options(Box::new(()))
                .with_credentials(|_| Ok(None)))
        })
    });
    let error = assert_git_error(result);
    assert!(error.contains("retained"));
    assert_eq!(fs::read(destination.join("tracked.txt")).unwrap(), b"QA599 owned collision\n");
    assert_eq!(fs::read(destination.join("keep-me")).unwrap(), b"QA599 owned sentinel\n");
    assert!(destination.join(".git").is_dir());
    assert_received(&destination, source.path(), "refs/remotes/origin/main");
    assert!(
        !destination.join(".git/index").exists(),
        "incomplete checkout cannot publish its index"
    );
    eprintln!("collision witnesses: foreign bytes and downloaded objects/tracking ref retained; initial index absent");

    let retained = snapshot(&destination);
    let retry = ProjectRepo::clone(
        &destination,
        &remote(),
        &cade_github::Host::Server(Url::parse("https://forge.example.invalid/").unwrap()),
        &octocrab::secrecy::SecretString::from("QA599_SYNTHETIC".to_owned()),
        Some(&cade_policy::Policy::default()),
        gix::progress::Discard,
        &AtomicBool::new(false),
    );
    assert!(retry.is_err(), "public re-clone must not replace occupied retained state");
    assert_eq!(snapshot(&destination), retained);

    // Explicit owner correction and recovery on this SAME repository. Do not
    // synthesize an index, reset HEAD, or weaken checkout's born-HEAD guard.
    fs::rename(destination.join("tracked.txt"), destination.join("saved-owned-file")).unwrap();
    let reopened = ProjectRepo::open(&destination).unwrap();
    fetch(&reopened, source.path()).expect("corrected explicit fetch");
    let target = sdk(source.path()).head_id().unwrap().detach();
    reopened.create_branch("refs/heads/qa599-recovered", target, actor()).unwrap();
    assert_eq!(fs::read(destination.join("saved-owned-file")).unwrap(), b"QA599 owned collision\n");
    assert_eq!(fs::read(destination.join("keep-me")).unwrap(), b"QA599 owned sentinel\n");
    assert!(!destination.join(".git/index").exists());
    eprintln!(
        "recovery witnesses: occupied public re-clone rejected unchanged; owner moved collision; same repo reopened/fetched/branched"
    );
    let checkout = reopened.checkout("refs/heads/qa599-recovered", actor());
    assert!(
        checkout.is_ok(),
        "retained clone must support same-repository explicit checkout after owner correction: {checkout:?}"
    );
    assert_eq!(fs::read(destination.join("tracked.txt")).unwrap(), b"remote revision\n");
    assert_eq!(fs::read(destination.join("saved-owned-file")).unwrap(), b"QA599 owned collision\n");
    assert_eq!(fs::read(destination.join("keep-me")).unwrap(), b"QA599 owned sentinel\n");
    assert!(destination.join(".git/index").is_file());
    assert_eq!(sdk(&destination).head_id().unwrap().detach(), target);
    assert_eq!(reopened.branch().unwrap().as_deref(), Some("qa599-recovered"));
}

const CANARY: &str = "QA599_REMOTE_CANARY_NOT_A_CREDENTIAL";

// Reader-only instrumentation, following InterruptRead and the independent QA
// ObserveRead. Single-byte reads witness transient task names before another
// sideband line replaces them; the bounded tail independently witnesses wire
// delivery. No protocol bytes, credentials or transport behavior are rewritten.
struct ObserveRead<R> {
    inner: R,
    progress: Arc<gix::progress::prodash::tree::Root>,
    leaked: Arc<AtomicBool>,
    sent: Arc<AtomicBool>,
    tail: Vec<u8>,
}

fn progress_contains_canary(root: &gix::progress::prodash::tree::Root) -> bool {
    let mut tasks = Vec::new();
    root.sorted_snapshot(&mut tasks);
    let mut messages = Vec::new();
    root.copy_messages(&mut messages);
    tasks.iter().any(|(_, task)| task.name.contains(CANARY))
        || messages
            .iter()
            .any(|message| message.message.contains(CANARY) || message.origin.contains(CANARY))
}

impl<R: Read> Read for ObserveRead<R> {
    fn read(&mut self, bytes: &mut [u8]) -> std::io::Result<usize> {
        if progress_contains_canary(&self.progress) {
            self.leaked.store(true, Ordering::SeqCst);
        }
        let size = bytes.len().min(1);
        let count = self.inner.read(&mut bytes[..size])?;
        self.tail.extend_from_slice(&bytes[..count]);
        if self.tail.windows(CANARY.len()).any(|window| window == CANARY.as_bytes()) {
            self.sent.store(true, Ordering::SeqCst);
        }
        if self.tail.len() > 128 {
            self.tail.drain(..self.tail.len() - 128);
        }
        Ok(count)
    }
}

#[test]
fn fetch_with_transport_server_sideband_never_reaches_progress_diagnostics() {
    let source = committed_repo();
    let local = committed_repo();
    advance(source.path());
    let repo = ProjectRepo::open(local.path()).unwrap();
    let before = selection(local.path());
    let root = gix::progress::prodash::tree::Root::new();
    let leaked = Arc::new(AtomicBool::new(false));
    let sent = Arc::new(AtomicBool::new(false));
    let marker_dir = tempfile::tempdir().unwrap();
    let marker = marker_dir.path().join("pack-hook-entered");
    // Git itself interprets packObjectsHook as a shell command and appends the
    // original `git pack-objects ...` arguments. Use only its existing shell
    // builtins and Git: no Python, script path, chmod, or new executable.
    // This protected config applies to the fixture SERVER only, never a client
    // filter/authentication helper. Git writes the marker outside either repo.
    let hook = format!(
        "uploadpack.packObjectsHook=fixture_pack () {{ \
         test \"$1\" = git && test \"$2\" = pack-objects || exit 97; \
         git config --file \"$CADE_GIT_PACK_HOOK_MARKER\" fixture.entered true && \
         printf '%s\\n' '{CANARY}' >&2 && exec \"$@\"; \
         }}; fixture_pack"
    );
    let (result, native) = with_upload_pack_io(
        source.path(),
        |command| {
            command.args(["-c", &hook]).env("CADE_GIT_PACK_HOOK_MARKER", &marker);
        },
        |stdout| {
            Box::new(ObserveRead {
                inner: stdout,
                progress: root.clone(),
                leaked: leaked.clone(),
                sent: sent.clone(),
                tail: Vec::new(),
            })
        },
        |transport| repo.fetch_with_transport(&remote(), transport, root.add_child("synthetic fetch"), &AtomicBool::new(false)),
    );
    eprintln!(
        "fixture server exit: {}; native stderr: {}",
        native.status,
        String::from_utf8_lossy(&native.stderr)
    );
    // Setup failures are not product reds. All witnesses precede the no-leak
    // assertion, as in the independent original.
    assert!(marker.is_file(), "SETUP: pack hook was not entered");
    assert!(native.status.success(), "SETUP: real pack-objects must complete");
    assert!(
        sent.load(Ordering::SeqCst),
        "SETUP: canary did not traverse real Git server protocol"
    );
    assert!(result.is_ok(), "ordinary transfer with server progress must still receive objects");
    assert!(matches!(result.unwrap().status, gix::remote::fetch::Status::Change { .. }));
    assert_received(local.path(), source.path(), "refs/remotes/origin/main");
    assert_eq!(selection(local.path()), before);
    eprintln!("progress witnesses: pack hook entered; real pack received; wire canary observed; local selection unchanged");
    assert!(
        !leaked.load(Ordering::SeqCst) && !progress_contains_canary(&root),
        "server-controlled text escaped into caller-visible progress diagnostics"
    );
}
}
