//! Filesystem/protocol integration probes with private SDK transport injection.
//! Like checkout_boundaries, these live under tests but compile in the library
//! test target. They are not evidence for the public HTTPS/policy/TLS boundary.

use crate::{GitError, ProjectRepo};
use gix::protocol::transport::{
    Protocol,
    client::{blocking_io::Transport, git::ConnectMode},
};
use std::{
    fs,
    io::Read,
    path::Path,
    process::{Command, Stdio},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
use url::Url;

use crate::support::{assert_git_error, committed_repo, git, run_isolated, snapshot};

mod clone;
mod fetch;
mod regressions;

pub(super) mod push {
    use super::*;

    thread_local! {
        pub(in crate::transport) static DESTINATION: std::cell::RefCell<Option<std::path::PathBuf>> = const { std::cell::RefCell::new(None) };
        pub(in crate::transport) static RECEIVE_URL: std::cell::RefCell<Option<String>> = const { std::cell::RefCell::new(None) };
        pub(in crate::transport) static QUERIES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
        pub(in crate::transport) static NEGOTIATIONS: std::cell::RefCell<Vec<(git2::Oid, git2::Oid, Vec<u8>)>> = const { std::cell::RefCell::new(Vec::new()) };
        pub(in crate::transport) static AFTER_VALIDATION: std::cell::RefCell<Option<Box<dyn FnOnce()>>> = const { std::cell::RefCell::new(None) };
        pub(in crate::transport) static BEFORE_NEGOTIATION: std::cell::RefCell<Option<Box<dyn FnOnce()>>> = const { std::cell::RefCell::new(None) };
        pub(in crate::transport) static AFTER_UPLOAD: std::cell::RefCell<Option<Box<dyn FnOnce()>>> = const { std::cell::RefCell::new(None) };
        pub(in crate::transport) static WIRES: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) };
        pub(in crate::transport) static UPLOAD_RESULTS: std::cell::RefCell<Vec<bool>> = const { std::cell::RefCell::new(Vec::new()) };
        pub(in crate::transport) static STATUSES: std::cell::RefCell<Vec<(String, Option<String>)>> = const { std::cell::RefCell::new(Vec::new()) };
        pub(in crate::transport) static STATUS_OVERRIDE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
        pub(in crate::transport) static STATUS_CALLBACK_ERROR: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
        pub(in crate::transport) static STATUS_INPUT: std::cell::RefCell<Vec<(String, Option<String>)>> = const { std::cell::RefCell::new(Vec::new()) };
        pub(in crate::transport) static NEGOTIATION_COUNT: std::cell::Cell<Option<usize>> = const { std::cell::Cell::new(None) };
        pub(in crate::transport) static NEGOTIATION_SOURCE: std::cell::RefCell<Vec<u8>> = const { std::cell::RefCell::new(Vec::new()) };
        pub(in crate::transport) static NEGOTIATION_DESTINATION: std::cell::RefCell<Vec<u8>> = const { std::cell::RefCell::new(Vec::new()) };
        pub(in crate::transport) static NEGOTIATION_OID: std::cell::Cell<Option<git2::Oid>> = const { std::cell::Cell::new(None) };
        pub(in crate::transport) static BEFORE_QUERY: std::cell::RefCell<Option<Box<dyn FnOnce()>>> = const { std::cell::RefCell::new(None) };
        pub(in crate::transport) static QUERY_OVERRIDE_AT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
        pub(in crate::transport) static QUERY_DISCONNECTED_AT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
        pub(in crate::transport) static QUERY_REFS: std::cell::RefCell<Vec<gix::protocol::handshake::Ref>> = const { std::cell::RefCell::new(Vec::new()) };
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    fn existing_destination(changed_selection: bool) {
        let dir = committed_repo();
        let parent = sdk(dir.path()).head_id().unwrap().detach();
        let destination = tempfile::tempdir().unwrap();
        let bare = git2::Repository::init_bare(destination.path().join(".git")).unwrap();
        let seed = git2::Repository::open(dir.path()).unwrap();
        seed.remote_anonymous(bare.path().to_str().unwrap())
            .unwrap()
            .push(&[format!("{parent}:refs/heads/selected")], None)
            .unwrap();
        let expected = advance(dir.path());
        let repo = ProjectRepo::open(dir.path()).unwrap();
        let mut before = snapshot(dir.path());
        if changed_selection {
            let path = dir.path().join(".git/refs/heads/main");
            let replacement = format!("{parent}\n").into_bytes();
            before.get_mut(Path::new(".git/refs/heads/main")).unwrap().1 = replacement.clone();
            AFTER_VALIDATION.set(Some(Box::new(move || {
                assert!(path.with_extension("lock").exists(), "no-edit source guard must still be held");
                fs::write(path, replacement).unwrap();
            })));
        }
        DESTINATION.set(Some(destination.path().to_owned()));
        QUERIES.set(0);
        NEGOTIATIONS.with_borrow_mut(Vec::clear);
        WIRES.with_borrow_mut(Vec::clear);
        STATUSES.with_borrow_mut(Vec::clear);
        let result = repo.push(
            "refs/heads/main",
            expected,
            "refs/heads/selected",
            &Url::parse("https://github.com/synthetic/project.git").unwrap(),
            &cade_github::Host::Dotcom,
            &octocrab::secrecy::SecretString::from("synthetic-token".to_owned()),
            Some(&cade_policy::Policy::default()),
            gix::progress::Discard,
            &AtomicBool::new(false),
        );
        DESTINATION.set(None);
        assert!(
            result.is_ok(),
            "actual non-force update must verify immutable selection: {result:?}"
        );
        assert_eq!(result.unwrap(), expected);
        assert_eq!(QUERIES.get(), 2);
        WIRES.with_borrow(|seen| assert_eq!(seen, &[format!("{expected}:refs/heads/selected")]));
        STATUSES.with_borrow(|seen| assert_eq!(seen, &[("refs/heads/selected".to_owned(), None)]));
        NEGOTIATIONS.with_borrow(|seen| {
            assert_eq!(seen.len(), 1);
            assert_eq!(seen[0].0.as_bytes(), parent.as_slice(), "nonzero advertised old OID");
            assert_eq!(seen[0].1.as_bytes(), expected.as_slice(), "immutable proposed new OID");
            assert_eq!(seen[0].2, expected.to_string().as_bytes());
        });
        assert_eq!(bare.refname_to_id("refs/heads/selected").unwrap().as_bytes(), expected.as_slice());
        assert_eq!(bare.references().unwrap().count(), 1);
        let source = sdk(dir.path());
        let observed = sdk(&destination.path().join(".git"));
        let commit = source.find_commit(expected).unwrap();
        assert_eq!(observed.find_object(expected).unwrap().data, commit.data);
        assert_eq!(commit.parent_ids().map(|id| id.detach()).collect::<Vec<_>>(), vec![parent]);
        let tree = commit.tree().unwrap();
        assert_eq!(observed.find_object(tree.id).unwrap().data, tree.data);
        for entry in tree.iter() {
            let id = entry.unwrap().oid().to_owned();
            assert_eq!(observed.find_object(id).unwrap().data, source.find_object(id).unwrap().data);
        }
        assert_eq!(
            source.head_id().unwrap().detach(),
            if changed_selection { parent } else { expected }
        );
        assert_eq!(
            snapshot(dir.path()),
            before,
            "only the explicit fixture writer's ref change is permitted"
        );
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    fn assert_push_preserves_repository(path: &Path) -> tempfile::TempDir {
        let expected = gix::ObjectId::from_hex(git(path, &["rev-parse", "refs/heads/main"]).trim_ascii()).unwrap();
        let destination = tempfile::tempdir().unwrap();
        let bare = git2::Repository::init_bare(destination.path().join(".git")).unwrap();
        let repo = ProjectRepo::open(path).unwrap();
        let before = snapshot(path);
        DESTINATION.set(Some(destination.path().to_owned()));
        QUERIES.set(0);
        WIRES.with_borrow_mut(Vec::clear);
        STATUSES.with_borrow_mut(Vec::clear);
        UPLOAD_RESULTS.with_borrow_mut(Vec::clear);
        let result = repo.push(
            "refs/heads/main",
            expected,
            "refs/heads/selected",
            &Url::parse("https://github.com/synthetic/project.git").unwrap(),
            &cade_github::Host::Dotcom,
            &octocrab::secrecy::SecretString::from("synthetic-token".to_owned()),
            Some(&cade_policy::Policy::default()),
            gix::progress::Discard,
            &AtomicBool::new(false),
        );
        DESTINATION.set(None);
        assert_eq!(result.unwrap(), expected);
        assert_eq!(QUERIES.get(), 2, "fresh gix observations before and after the SDK upload");
        WIRES.with_borrow(|seen| assert_eq!(seen, &[format!("{expected}:refs/heads/selected")]));
        STATUSES.with_borrow(|seen| assert_eq!(seen, &[("refs/heads/selected".to_owned(), None)]));
        UPLOAD_RESULTS.with_borrow(|seen| assert_eq!(seen, &[true]));
        assert_eq!(bare.refname_to_id("refs/heads/selected").unwrap().as_bytes(), expected.as_slice());
        assert_eq!(bare.references().unwrap().count(), 1, "no stash or other branch published");
        let source = sdk(path);
        let observed = sdk(&destination.path().join(".git"));
        let objects = String::from_utf8(git(path, &["rev-list", "--objects", &expected.to_string()])).unwrap();
        assert!(!objects.is_empty());
        for line in objects.lines() {
            let id = gix::ObjectId::from_hex(line.split_whitespace().next().unwrap().as_bytes()).unwrap();
            let local = source.find_object(id).unwrap();
            let remote = observed.find_object(id).unwrap();
            assert_eq!(remote.kind, local.kind);
            assert_eq!(remote.data, local.data, "all reachable commit/tree/blob bytes, including ancestry");
        }
        assert_eq!(
            git(destination.path(), &["rev-list", "--parents", "refs/heads/selected"]),
            git(path, &["rev-list", "--parents", "refs/heads/main"]),
        );
        assert_eq!(
            snapshot(path),
            before,
            "no filtering: HEAD/index/config/objects/refs/reflogs/worktree all preserved"
        );
        destination
    }

    #[cfg(unix)]
    fn receive_pack(destination: &Path) -> (String, std::thread::JoinHandle<(u32, std::process::Output)>) {
        use std::os::fd::OwnedFd;
        let listener = std::net::TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let address = listener.local_addr().unwrap();
        listener.set_nonblocking(true).unwrap();
        let server_path = destination.to_owned();
        let server = std::thread::spawn(move || {
            let start = std::time::Instant::now();
            let stream = loop {
                match listener.accept() {
                    Ok((stream, peer)) => {
                        assert!(peer.ip().is_loopback());
                        break stream;
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        assert!(
                            start.elapsed() < std::time::Duration::from_secs(10),
                            "owned receive connection timed out"
                        );
                        std::thread::sleep(std::time::Duration::from_millis(10));
                    }
                    Err(error) => panic!("owned receive listener: {error}"),
                }
            };
            let input: OwnedFd = stream.try_clone().unwrap().into();
            let output: OwnedFd = stream.into();
            let mut child = Command::new("git")
                .env_clear()
                .env("PATH", std::env::var_os("PATH").unwrap())
                .env("HOME", &server_path)
                .env("GIT_CONFIG_NOSYSTEM", "1")
                .env("GIT_CONFIG_GLOBAL", "/dev/null")
                .args(["daemon", "--inetd", "--export-all", "--enable=receive-pack"])
                .arg(format!("--base-path={}", server_path.display()))
                .stdin(Stdio::from(input))
                .stdout(Stdio::from(output))
                .stderr(Stdio::piped())
                .spawn()
                .unwrap();
            let pid = child.id();
            let start = std::time::Instant::now();
            loop {
                if child.try_wait().unwrap().is_some() {
                    break;
                }
                if start.elapsed() > std::time::Duration::from_secs(10) {
                    child.kill().unwrap();
                    let output = child.wait_with_output().unwrap();
                    panic!("owned receive-pack PID {pid} timed out: {:?}", output.status);
                }
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
            (pid, child.wait_with_output().unwrap())
        });
        (format!("git://{address}/.git"), server)
    }

    isolated_tests! {
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn push_workflow_retains_content_and_trailers() {
        let origin = committed_repo();
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("workflow with space Ω");
        let interrupt = AtomicBool::new(false);
        let (cloned, _) = with_upload_pack(origin.path(), None, |transport| {
            ProjectRepo::clone_with_transport(&path, &remote(), transport, gix::progress::Discard, &interrupt)
        });
        let repo = cloned.unwrap();
        let source = sdk(&path);
        let parent = source.head_id().unwrap().detach();
        assert_eq!(parent, sdk(origin.path()).head_id().unwrap().detach());
        let mut commit = source.find_commit(parent).unwrap().decode().unwrap().to_owned().unwrap();
        let blob = source.write_blob(b"explicit workflow document\n").unwrap().detach();
        commit.tree = source.write_object(&gix::objs::Tree {
            entries: vec![gix::objs::tree::Entry {
                mode: gix::objs::tree::EntryKind::Blob.into(),
                filename: "workflow.cade".into(),
                oid: blob,
            }],
        }).unwrap().detach();
        commit.parents = vec![parent].into();
        let message = crate::run_commit_message("Apply synthetic workflow", "run-workflow", "synthetic-model", 12345);
        commit.message = message.clone().into();
        let expected = repo.commit("refs/heads/main", &commit).unwrap();
        repo.create_branch("refs/heads/workflow", expected, commit.committer.to_ref(&mut gix::date::parse::TimeBuf::default())).unwrap();
        let before = snapshot(&path);
        let destination = tempfile::tempdir().unwrap();
        git2::Repository::init_bare(destination.path().join(".git")).unwrap();
        DESTINATION.set(Some(destination.path().to_owned()));
        QUERIES.set(0);
        WIRES.with_borrow_mut(Vec::clear);
        let result = repo.push(
            "refs/heads/workflow", expected, "refs/heads/review",
            &Url::parse("https://github.com/synthetic/project.git").unwrap(), &cade_github::Host::Dotcom,
            &octocrab::secrecy::SecretString::from("synthetic-token".to_owned()),
            Some(&cade_policy::Policy::default()), gix::progress::Discard, &interrupt,
        );
        DESTINATION.set(None);
        assert_eq!(result.unwrap(), expected);
        assert_eq!(QUERIES.get(), 2, "fresh pre/post observations, not cached clone state");
        WIRES.with_borrow(|seen| assert_eq!(seen, &[format!("{expected}:refs/heads/review")]));
        let observed = sdk(&destination.path().join(".git"));
        assert_eq!(observed.find_reference("refs/heads/review").unwrap().try_id().unwrap().detach(), expected);
        let stored = observed.find_commit(expected).unwrap();
        assert_eq!(stored.data, source.find_commit(expected).unwrap().data);
        assert_eq!(stored.parent_ids().map(|id| id.detach()).collect::<Vec<_>>(), vec![parent]);
        assert_eq!(observed.find_object(parent).unwrap().data, source.find_object(parent).unwrap().data);
        assert_eq!(stored.tree().unwrap().id, commit.tree);
        assert_eq!(observed.find_object(commit.tree).unwrap().data, source.find_object(commit.tree).unwrap().data);
        assert_eq!(observed.find_object(blob).unwrap().data, b"explicit workflow document\n");
        assert_eq!(stored.message_raw().unwrap(), message.as_str());
        for trailer in ["CADE-Copilot-Run: run-workflow\n", "CADE-Model: synthetic-model\n", "CADE-Tokens: 12345\n"] {
            assert!(message.contains(trailer), "all required trailer values must survive: {trailer}");
        }
        assert_eq!(snapshot(&path), before, "push must not change branch, HEAD, index or worktree");
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn push_pull_request_policy_allows_branch_upload() {
        use cade_policy::{Action, Decision, Policy, ShipVia, Switch};
        for network in [Switch::On, Switch::Off] {
            let dir = committed_repo();
            let repo = ProjectRepo::open(dir.path()).unwrap();
            let expected = sdk(dir.path()).head_id().unwrap().detach();
            let destination = tempfile::tempdir().unwrap();
            git2::Repository::init_bare(destination.path().join(".git")).unwrap();
            let before = snapshot(dir.path());
            let remote_before = snapshot(destination.path());
            let mut policy = Policy::default();
            policy.tools.network = network;
            policy.outputs.ship_via = ShipVia::PullRequest;
            assert!(matches!(policy.decide(&Action::ShipByPush), Decision::Deny { .. }),
                "branch upload never authorizes direct shipping");
            DESTINATION.set(Some(destination.path().to_owned()));
            QUERIES.set(0);
            WIRES.with_borrow_mut(Vec::clear);
            let result = repo.push(
                "refs/heads/main", expected, "refs/heads/review",
                &Url::parse("https://github.com/synthetic/project.git").unwrap(), &cade_github::Host::Dotcom,
                &octocrab::secrecy::SecretString::from("synthetic-token".to_owned()),
                Some(&policy), gix::progress::Discard, &AtomicBool::new(false),
            );
            DESTINATION.set(None);
            match network {
                Switch::On => {
                    assert_eq!(policy.decide(&Action::AgentNetwork), Decision::Allow);
                    assert_eq!(result.unwrap(), expected, "only branch mechanics, not PR creation or Ship completion");
                    assert_eq!(QUERIES.get(), 2);
                    WIRES.with_borrow(|seen| assert_eq!(seen, &[format!("{expected}:refs/heads/review")]));
                    let observed = sdk(&destination.path().join(".git"));
                    assert_eq!(observed.find_reference("refs/heads/review").unwrap().try_id().unwrap().detach(), expected);
                    assert_eq!(observed.find_object(expected).unwrap().data, sdk(dir.path()).find_object(expected).unwrap().data);
                }
                Switch::Off => {
                    assert!(matches!(policy.decide(&Action::AgentNetwork), Decision::Deny { .. }));
                    assert_git_error(result);
                    assert_eq!(QUERIES.get(), 0, "denial precedes pre-query");
                    WIRES.with_borrow(|seen| assert!(seen.is_empty(), "denial precedes SDK upload"));
                    assert_eq!(snapshot(destination.path()), remote_before);
                }
            }
            assert_eq!(snapshot(dir.path()), before);
        }
    }

    #[cfg(windows)]
    #[test]
    fn git_push_remote_windows_backend_prevents_network() {
        let dir = committed_repo();
        let repo = ProjectRepo::open(dir.path()).unwrap();
        let expected = sdk(dir.path()).head_id().unwrap().detach();
        let private = git2::Repository::open(dir.path()).unwrap();
        let address = Url::parse("https://github.com/synthetic/project.git").unwrap();
        let host = cade_github::Host::Dotcom;
        let token = octocrab::secrecy::SecretString::from("synthetic-token".to_owned());
        let policy = cade_policy::Policy::default();
        let interrupt = AtomicBool::new(false);
        let before = snapshot(dir.path());
        let result = host.git_push_remote(&private, &address, &token, Some(&policy), git2::RemoteCallbacks::new(), &interrupt);
        assert!(matches!(result, Err(cade_github::GitHubError::GitTransport("secure Git push is unsupported on this platform"))));
        let destination = tempfile::tempdir().unwrap();
        git2::Repository::init_bare(destination.path().join(".git")).unwrap();
        let remote_before = snapshot(destination.path());
        DESTINATION.set(Some(destination.path().to_owned()));
        QUERIES.set(0);
        WIRES.with_borrow_mut(Vec::clear);
        let message = assert_git_error(repo.push(
            "refs/heads/main", expected, "refs/heads/review",
            &address, &host, &token, Some(&policy), gix::progress::Discard, &interrupt,
        ));
        DESTINATION.set(None);
        assert_eq!(message, "secure Git push is unsupported on this platform");
        assert_eq!(QUERIES.get(), 0, "unsupported factory must precede every remote pre-query");
        WIRES.with_borrow(|seen| assert!(seen.is_empty()));
        assert_eq!(snapshot(destination.path()), remote_before);
        assert_eq!(snapshot(dir.path()), before);
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn push_numeric_progress_reports_verified_completion() {
        let dir = committed_repo();
        let expected = sdk(dir.path()).head_id().unwrap().detach();
        let destination = tempfile::tempdir().unwrap();
        let bare = git2::Repository::init_bare(destination.path().join(".git")).unwrap();
        let before = snapshot(dir.path());
        let progress = gix::progress::prodash::tree::Root::new();
        let (receive_url, server) = receive_pack(destination.path());
        RECEIVE_URL.set(Some(receive_url));
        let captured = Arc::new(std::sync::Mutex::new(Vec::new()));
        let during_upload = Arc::clone(&captured);
        let observer = Arc::clone(&progress);
        AFTER_UPLOAD.set(Some(Box::new(move || {
            observer.sorted_snapshot(&mut during_upload.lock().unwrap());
            let mut messages = Vec::new();
            observer.copy_messages(&mut messages);
            assert!(messages.iter().all(|message| message.level != gix::progress::prodash::messages::MessageLevel::Success),
                "upload completion alone cannot report verified completion");
        })));
        DESTINATION.set(Some(destination.path().to_owned()));
        QUERIES.set(0);
        let result = ProjectRepo::open(dir.path()).unwrap().push(
            "refs/heads/main", expected, "refs/heads/selected",
            &Url::parse("https://github.com/synthetic/project.git").unwrap(),
            &cade_github::Host::Dotcom, &octocrab::secrecy::SecretString::from("synthetic-token".to_owned()),
            Some(&cade_policy::Policy::default()), progress.add_child("synthetic push"), &AtomicBool::new(false));
        DESTINATION.set(None);
        RECEIVE_URL.set(None);
        let (_, output) = server.join().unwrap();
        assert!(output.status.success());
        assert_eq!(result.unwrap(), expected);
        assert_eq!(QUERIES.get(), 2);
        let tasks = captured.lock().unwrap();
        for name in ["Git push pack objects", "Git push upload objects", "Git push upload bytes", "Git push verified refs"] {
            let task = tasks.iter().find(|(_, task)| task.name == name);
            assert!(task.is_some(), "missing production numeric progress child: {name}");
            let value = task.unwrap().1.progress.as_ref().unwrap();
            assert!(value.step.load(Ordering::Relaxed) > 0, "SDK callback count must be nonzero: {name}");
            if name == "Git push verified refs" {
                assert_eq!(value.step.load(Ordering::Relaxed), 1);
                assert_eq!(value.done_at, Some(1));
            }
        }
        let mut messages = Vec::new();
        progress.copy_messages(&mut messages);
        assert_eq!(messages.iter().filter(|message| message.level == gix::progress::prodash::messages::MessageLevel::Success).count(), 1);
        assert_eq!(bare.refname_to_id("refs/heads/selected").unwrap().as_bytes(), expected.as_slice());
        assert_eq!(snapshot(dir.path()), before);
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn push_conflicted_worktree_preserves_unselected_state() {
        let dir = committed_repo();
        let base = sdk(dir.path()).head_id().unwrap().detach();
        let ours = advance(dir.path());
        git(dir.path(), &["checkout", "-q", "-b", "conflict-side", &base.to_string()]);
        fs::write(dir.path().join("tracked.txt"), b"unselected conflicting side\n").unwrap();
        git(dir.path(), &["add", "--", "tracked.txt"]);
        git(dir.path(), &["commit", "-q", "-m", "Unselected side"]);
        let theirs = sdk(dir.path()).head_id().unwrap().detach();
        let unselected_blob = gix::ObjectId::from_hex(git(dir.path(), &["rev-parse", "HEAD:tracked.txt"]).trim_ascii()).unwrap();
        git(dir.path(), &["checkout", "-q", "main"]);
        git(dir.path(), &["read-tree", "-m", &base.to_string(), &ours.to_string(), &theirs.to_string()]);
        let stages = String::from_utf8(git(dir.path(), &["ls-files", "--unmerged"])).unwrap();
        assert_eq!(stages.lines().count(), 3, "real base/ours/theirs index entries");
        for stage in [1, 2, 3] {
            assert!(stages.contains(&format!(" {stage}\ttracked.txt\n")));
        }
        fs::write(dir.path().join("tracked.txt"), b"unresolved worktree edits, not committed\n").unwrap();
        fs::write(dir.path().join("untracked.txt"), b"unselected untracked bytes\n").unwrap();
        fs::create_dir(dir.path().join("empty directory")).unwrap();
        std::os::unix::fs::symlink("tracked.txt", dir.path().join("owned-link")).unwrap();
        let destination = assert_push_preserves_repository(dir.path());
        let observed = sdk(&destination.path().join(".git"));
        assert!(observed.find_object(theirs).is_err(), "unselected side commit must not be uploaded");
        assert!(observed.find_object(unselected_blob).is_err(), "unselected conflict blob must not be uploaded");
        assert_eq!(String::from_utf8(git(dir.path(), &["ls-files", "--unmerged"])).unwrap(), stages);
        println!("row31: real three-stage conflict, edited worktree, untracked file, empty directory and symlink preserved");
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn push_explicit_branch_ignores_head_attachment() {
        for attachment in ["bare", "detached", "unborn"] {
            let dir = committed_repo();
            let parent = sdk(dir.path()).head_id().unwrap().detach();
            advance(dir.path());
            let bare = tempfile::tempdir().unwrap();
            let path = match attachment {
                "bare" => {
                    git(dir.path(), &["clone", "-q", "--bare", dir.path().to_str().unwrap(), bare.path().to_str().unwrap()]);
                    assert_eq!(git(bare.path(), &["rev-parse", "--is-bare-repository"]), b"true\n");
                    bare.path()
                }
                "detached" => {
                    git(dir.path(), &["checkout", "-q", "--detach", &parent.to_string()]);
                    assert_eq!(sdk(dir.path()).head_id().unwrap().detach(), parent);
                    assert_eq!(fs::read(dir.path().join(".git/HEAD")).unwrap(), format!("{parent}\n").as_bytes());
                    dir.path()
                }
                "unborn" => {
                    git(dir.path(), &["symbolic-ref", "HEAD", "refs/heads/unborn"]);
                    assert!(!dir.path().join(".git/refs/heads/unborn").exists());
                    assert_eq!(fs::read(dir.path().join(".git/HEAD")).unwrap(), b"ref: refs/heads/unborn\n");
                    dir.path()
                }
                _ => unreachable!(),
            };
            assert_push_preserves_repository(path);
            println!("row32: {attachment} HEAD preserved while explicit main uploaded");
        }
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn push_opaque_document_blob_preserves_bytes() {
        // Generated with pinned Loro 1.17.1 and independently opened as a
        // Project with manifest.format_version == 0. Push never imports a
        // document library: these fixed bytes are solely an opaque Git blob.
        let older: &[u8] = &[
            108,111,114,111,0,0,0,0,0,0,0,0,0,0,0,0,13,41,226,48,0,3,242,0,0,0,76,79,82,79,0,0,1,0,1,1,16,1,1,0,0,0,0,0,0,0,
            1,1,0,0,0,0,0,5,1,0,0,1,0,6,1,4,1,0,0,2,14,4,106,115,111,110,8,109,97,110,105,102,101,115,116,0,14,1,4,2,1,0,2,1,0,
            2,1,11,2,1,1,0,106,5,104,123,34,102,111,114,109,97,116,95,118,101,114,115,105,111,110,34,58,48,44,34,107,105,110,100,34,58,34,
            100,111,99,117,109,101,110,116,34,44,34,116,105,116,108,101,34,58,34,83,121,110,116,104,101,116,105,99,32,111,108,100,101,114,32,
            100,111,99,117,109,101,110,116,34,44,34,100,101,115,105,103,110,95,115,121,115,116,101,109,115,34,58,91,93,44,34,108,105,110,107,
            115,34,58,91,93,125,0,2,0,102,114,1,1,0,0,2,0,118,118,1,1,2,0,0,174,0,182,0,3,0,96,60,255,202,1,0,0,0,5,0,0,0,12,0,0,0,
            0,0,0,0,0,1,0,0,0,0,0,2,0,118,118,150,40,213,135,207,0,0,0,181,0,0,0,76,79,82,79,0,0,1,0,1,4,106,115,111,110,4,104,123,34,
            102,111,114,109,97,116,95,118,101,114,115,105,111,110,34,58,48,44,34,107,105,110,100,34,58,34,100,111,99,117,109,101,110,116,34,
            44,34,116,105,116,108,101,34,58,34,83,121,110,116,104,101,116,105,99,32,111,108,100,101,114,32,100,111,99,117,109,101,110,116,34,
            44,34,100,101,115,105,103,110,95,115,121,115,116,101,109,115,34,58,91,93,44,34,108,105,110,107,115,34,58,91,93,125,0,1,1,0,0,0,
            0,0,0,0,0,0,0,0,1,0,245,223,86,242,1,0,0,0,5,0,0,0,10,0,128,8,109,97,110,105,102,101,115,116,0,10,0,128,8,109,97,110,
            105,102,101,115,116,146,208,102,150,140,0,0,0,0,0,0,0,
        ];
        for (case, bytes) in [("older-version", older), ("malformed", b"loro\0\xfftruncated\r\n\0invalid-document".as_slice())] {
            let dir = committed_repo();
            fs::write(dir.path().join("synthetic.cade"), bytes).unwrap();
            git(dir.path(), &["add", "--", "synthetic.cade"]);
            git(dir.path(), &["commit", "-q", "-m", "Opaque synthetic document"]);
            let blob = gix::ObjectId::from_hex(git(dir.path(), &["rev-parse", "HEAD:synthetic.cade"]).trim_ascii()).unwrap();
            fs::write(dir.path().join("synthetic.cade"), b"unselected later document edits").unwrap();
            let destination = assert_push_preserves_repository(dir.path());
            assert_eq!(sdk(&destination.path().join(".git")).find_object(blob).unwrap().data, bytes);
            println!("row33: {case} committed blob exact bytes preserved; later worktree edits not selected");
        }
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn push_post_publication_sdk_error_requires_explicit_recovery() {
        // This is a real SDK top-level error caused by a callback after publication,
        // not a lost network acknowledgement or an injected Result from Remote::push.
        for disconnected in [false, true] {
            let dir = committed_repo();
            let expected = sdk(dir.path()).head_id().unwrap().detach();
            let destination = tempfile::tempdir().unwrap();
            let bare = git2::Repository::init_bare(destination.path().join(".git")).unwrap();
            let before = snapshot(dir.path());
            DESTINATION.set(Some(destination.path().to_owned()));
            QUERIES.set(0);
            QUERY_DISCONNECTED_AT.set(if disconnected { 2 } else { 0 });
            WIRES.with_borrow_mut(Vec::clear);
            UPLOAD_RESULTS.with_borrow_mut(Vec::clear);
            STATUSES.with_borrow_mut(Vec::clear);
            STATUS_CALLBACK_ERROR.set(true);
            let repo = ProjectRepo::open(dir.path()).unwrap();
            let remote = Url::parse("https://github.com/synthetic/project.git").unwrap();
            let token = octocrab::secrecy::SecretString::from("synthetic-token".to_owned());
            let policy = cade_policy::Policy::default();
            let first = repo.push("refs/heads/main", expected, "refs/heads/selected", &remote,
                &cade_github::Host::Dotcom, &token, Some(&policy), gix::progress::Discard, &AtomicBool::new(false));
            assert_eq!(first.unwrap_err().to_string(), if disconnected {
                "git: Git push outcome is uncertain; remote refs may have changed; verify before retrying"
            } else {
                "git: Git push failed; intended remote tip is now observed; reconcile before another action"
            });
            assert_eq!(QUERIES.get(), 2);
            UPLOAD_RESULTS.with_borrow(|seen| assert_eq!(seen, &[false], "actual SDK returned Err"));
            STATUSES.with_borrow(|seen| assert_eq!(seen, &[("refs/heads/selected".to_owned(), None)]));
            assert_eq!(bare.refname_to_id("refs/heads/selected").unwrap().as_bytes(), expected.as_slice(),
                "actual publication despite top-level SDK failure");
            assert_eq!(snapshot(dir.path()), before);
            let published = snapshot(destination.path());
            STATUS_CALLBACK_ERROR.set(false);
            QUERY_DISCONNECTED_AT.set(0);
            let recovered = repo.push("refs/heads/main", expected, "refs/heads/selected", &remote,
                &cade_github::Host::Dotcom, &token, Some(&policy), gix::progress::Discard, &AtomicBool::new(false));
            DESTINATION.set(None);
            assert_eq!(recovered.unwrap(), expected);
            assert_eq!(QUERIES.get(), 3, "new authorized request freshly observes actual remote");
            WIRES.with_borrow(|seen| assert_eq!(seen, &[format!("{expected}:refs/heads/selected")]));
            UPLOAD_RESULTS.with_borrow(|seen| assert_eq!(seen, &[false], "no duplicate upload or automatic retry"));
            assert_eq!(snapshot(dir.path()), before);
            assert_eq!(snapshot(destination.path()), published, "never roll back published state");
        }
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn push_recovery_observation_prevents_duplicate_upload() {
        let dir = committed_repo();
        let expected = sdk(dir.path()).head_id().unwrap().detach();
        let destination = tempfile::tempdir().unwrap();
        let bare = git2::Repository::init_bare(destination.path().join(".git")).unwrap();
        let before = snapshot(dir.path());
        let interrupt = Arc::new(AtomicBool::new(false));
        let flag = Arc::clone(&interrupt);
        let remote_path = bare.path().to_owned();
        AFTER_UPLOAD.set(Some(Box::new(move || {
            let actual = git2::Repository::open_bare(remote_path).unwrap();
            assert_eq!(actual.refname_to_id("refs/heads/selected").unwrap().as_bytes(), expected.as_slice());
            flag.store(true, Ordering::Relaxed);
        })));
        DESTINATION.set(Some(destination.path().to_owned()));
        QUERIES.set(0);
        WIRES.with_borrow_mut(Vec::clear);
        UPLOAD_RESULTS.with_borrow_mut(Vec::clear);
        let repo = ProjectRepo::open(dir.path()).unwrap();
        let remote = Url::parse("https://github.com/synthetic/project.git").unwrap();
        let token = octocrab::secrecy::SecretString::from("synthetic-token".to_owned());
        let policy = cade_policy::Policy::default();
        let first = repo.push("refs/heads/main", expected, "refs/heads/selected", &remote,
            &cade_github::Host::Dotcom, &token, Some(&policy), gix::progress::Discard, &interrupt);
        assert_eq!(first.unwrap_err().to_string(),
            "git: Git push outcome is uncertain; remote refs may have changed; verify before retrying");
        assert_eq!(QUERIES.get(), 1);
        assert_eq!(snapshot(dir.path()), before);
        let published = snapshot(destination.path());
        let usable = AtomicBool::new(false);
        let denied = repo.push("refs/heads/main", expected, "refs/heads/selected", &remote,
            &cade_github::Host::Dotcom, &token, None, gix::progress::Discard, &usable);
        assert!(denied.is_err(), "recovery requires newly supplied authorization");
        assert_eq!(QUERIES.get(), 1, "denied recovery cannot observe");
        BEFORE_QUERY.set(Some(Box::new(|| {
            assert_eq!(QUERIES.get(), 2, "explicit recovery starts a new actual gix observation");
            WIRES.with_borrow(|seen| assert_eq!(seen.len(), 1, "observe before any possible retransmission"));
        })));
        let recovered = repo.push("refs/heads/main", expected, "refs/heads/selected", &remote,
            &cade_github::Host::Dotcom, &token, Some(&policy), gix::progress::Discard, &usable);
        DESTINATION.set(None);
        assert_eq!(recovered.unwrap(), expected);
        assert_eq!(QUERIES.get(), 2);
        WIRES.with_borrow(|seen| assert_eq!(seen, &[format!("{expected}:refs/heads/selected")]));
        UPLOAD_RESULTS.with_borrow(|seen| assert_eq!(seen, &[true], "no duplicate upload"));
        assert!(interrupt.load(Ordering::Relaxed), "old canceled request is not silently reset");
        assert_eq!(snapshot(dir.path()), before);
        assert_eq!(snapshot(destination.path()), published);
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn push_failed_verification_retains_uncertainty() {
        let progress = gix::progress::prodash::tree::Root::new();
        let dir = committed_repo();
        let expected = sdk(dir.path()).head_id().unwrap().detach();
        let destination = tempfile::tempdir().unwrap();
        let bare = git2::Repository::init_bare(destination.path().join(".git")).unwrap();
        let before = snapshot(dir.path());
        DESTINATION.set(Some(destination.path().to_owned()));
        QUERIES.set(0);
        QUERY_DISCONNECTED_AT.set(2);
        WIRES.with_borrow_mut(Vec::clear);
        UPLOAD_RESULTS.with_borrow_mut(Vec::clear);
        STATUSES.with_borrow_mut(Vec::clear);
        let result = ProjectRepo::open(dir.path()).unwrap().push(
            "refs/heads/main", expected, "refs/heads/selected",
            &Url::parse("https://github.com/synthetic/project.git").unwrap(),
            &cade_github::Host::Dotcom, &octocrab::secrecy::SecretString::from("synthetic-token".to_owned()),
            Some(&cade_policy::Policy::default()), progress.add_child("synthetic push"), &AtomicBool::new(false));
        QUERY_DISCONNECTED_AT.set(0);
        DESTINATION.set(None);
        assert_eq!(result.unwrap_err().to_string(),
            "git: Git push outcome is uncertain; remote refs may have changed; verify before retrying");
        assert_eq!(QUERIES.get(), 2, "actual disconnected verification is attempted exactly once");
        UPLOAD_RESULTS.with_borrow(|seen| assert_eq!(seen, &[true], "real upload preceded failed verification"));
        WIRES.with_borrow(|seen| assert_eq!(seen, &[format!("{expected}:refs/heads/selected")], "never automatic retry"));
        STATUSES.with_borrow(|seen| assert_eq!(seen, &[("refs/heads/selected".to_owned(), None)]));
        assert_eq!(bare.refname_to_id("refs/heads/selected").unwrap().as_bytes(), expected.as_slice());
        assert_eq!(sdk(&destination.path().join(".git")).find_object(expected).unwrap().data,
            sdk(dir.path()).find_object(expected).unwrap().data);
        assert_eq!(snapshot(dir.path()), before, "failure releases only the owned source guard");
        let mut messages = Vec::new();
        progress.copy_messages(&mut messages);
        assert!(!messages.is_empty());
        assert!(messages.iter().all(|message| message.level != gix::progress::prodash::messages::MessageLevel::Success));
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn push_offline_attempt_preserves_local_work() {
        let dir = committed_repo();
        let expected = sdk(dir.path()).head_id().unwrap().detach();
        fs::write(dir.path().join("untracked-local-work"), b"unsaved synthetic work").unwrap();
        let destination = tempfile::tempdir().unwrap();
        let bare = git2::Repository::init_bare(destination.path().join(".git")).unwrap();
        let before = snapshot(dir.path());
        let remote_before = snapshot(destination.path());
        let repo = ProjectRepo::open(dir.path()).unwrap();
        let remote = Url::parse("https://github.com/synthetic/project.git").unwrap();
        let token = octocrab::secrecy::SecretString::from("synthetic-token".to_owned());
        let policy = cade_policy::Policy::default();
        DESTINATION.set(Some(destination.path().to_owned()));
        QUERIES.set(0);
        QUERY_DISCONNECTED_AT.set(1);
        WIRES.with_borrow_mut(Vec::clear);
        UPLOAD_RESULTS.with_borrow_mut(Vec::clear);
        let offline = repo.push("refs/heads/main", expected, "refs/heads/selected", &remote,
            &cade_github::Host::Dotcom, &token, Some(&policy), gix::progress::Discard, &AtomicBool::new(false));
        assert_eq!(offline.unwrap_err().to_string(), "git: fresh Git push observation failed");
        assert_eq!(QUERIES.get(), 1, "actual gix disconnected transport failure, not a preflight stub");
        WIRES.with_borrow(|seen| assert!(seen.is_empty()));
        assert_eq!(snapshot(dir.path()), before);
        assert_eq!(snapshot(destination.path()), remote_before);
        QUERY_DISCONNECTED_AT.set(0);
        BEFORE_QUERY.set(Some(Box::new(|| {
            assert_eq!(QUERIES.get(), 2);
            WIRES.with_borrow(|seen| assert!(seen.is_empty(), "corrected explicit attempt observes before uploading"));
        })));
        let corrected = repo.push("refs/heads/main", expected, "refs/heads/selected", &remote,
            &cade_github::Host::Dotcom, &token, Some(&policy), gix::progress::Discard, &AtomicBool::new(false));
        DESTINATION.set(None);
        assert_eq!(corrected.unwrap(), expected);
        assert_eq!(QUERIES.get(), 3);
        WIRES.with_borrow(|seen| assert_eq!(seen, &[format!("{expected}:refs/heads/selected")]));
        UPLOAD_RESULTS.with_borrow(|seen| assert_eq!(seen, &[true]));
        assert_eq!(bare.refname_to_id("refs/heads/selected").unwrap().as_bytes(), expected.as_slice());
        assert_eq!(snapshot(dir.path()), before, "explicit retry preserves uncommitted local work");
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn push_precanceled_request_prevents_transport() {
        let dir = committed_repo();
        let expected = sdk(dir.path()).head_id().unwrap().detach();
        let destination = tempfile::tempdir().unwrap();
        git2::Repository::init_bare(destination.path().join(".git")).unwrap();
        let before = snapshot(dir.path());
        let remote_before = snapshot(destination.path());
        DESTINATION.set(Some(destination.path().to_owned()));
        AFTER_VALIDATION.set(Some(Box::new(|| panic!("canceled entry must not acquire the source guard"))));
        QUERIES.set(0);
        WIRES.with_borrow_mut(Vec::clear);
        let interrupt = AtomicBool::new(true);
        let result = ProjectRepo::open(dir.path()).unwrap().push(
            "refs/heads/main", expected, "refs/heads/selected",
            &Url::parse("https://github.com/synthetic/project.git").unwrap(),
            &cade_github::Host::Dotcom, &octocrab::secrecy::SecretString::from("synthetic-token".to_owned()),
            Some(&cade_policy::Policy::default()), gix::progress::Discard, &interrupt);
        AFTER_VALIDATION.take();
        DESTINATION.set(None);
        assert!(result.is_err());
        assert!(format!("{result:?}").contains("interrupted"));
        assert_eq!(QUERIES.get(), 0);
        WIRES.with_borrow(|seen| assert!(seen.is_empty()));
        assert!(interrupt.load(Ordering::Relaxed));
        assert_eq!(snapshot(dir.path()), before);
        assert_eq!(snapshot(destination.path()), remote_before);
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn push_negotiation_interruption_prevents_upload() {
        let dir = committed_repo();
        let expected = sdk(dir.path()).head_id().unwrap().detach();
        let destination = tempfile::tempdir().unwrap();
        git2::Repository::init_bare(destination.path().join(".git")).unwrap();
        let before = snapshot(dir.path());
        let remote_before = snapshot(destination.path());
        let interrupt = Arc::new(AtomicBool::new(false));
        let flag = Arc::clone(&interrupt);
        BEFORE_NEGOTIATION.set(Some(Box::new(move || flag.store(true, Ordering::Relaxed))));
        DESTINATION.set(Some(destination.path().to_owned()));
        QUERIES.set(0);
        WIRES.with_borrow_mut(Vec::clear);
        UPLOAD_RESULTS.with_borrow_mut(Vec::clear);
        STATUSES.with_borrow_mut(Vec::clear);
        let result = ProjectRepo::open(dir.path()).unwrap().push(
            "refs/heads/main", expected, "refs/heads/selected",
            &Url::parse("https://github.com/synthetic/project.git").unwrap(),
            &cade_github::Host::Dotcom, &octocrab::secrecy::SecretString::from("synthetic-token".to_owned()),
            Some(&cade_policy::Policy::default()), gix::progress::Discard, &interrupt);
        DESTINATION.set(None);
        assert!(result.is_err());
        assert_eq!(format!("{}", result.unwrap_err()),
            "git: Git push outcome is uncertain; remote refs may have changed; verify before retrying");
        assert!(interrupt.load(Ordering::Relaxed));
        assert_eq!(QUERIES.get(), 1, "no post-cancel query");
        WIRES.with_borrow(|seen| assert_eq!(seen, &[format!("{expected}:refs/heads/selected")]));
        UPLOAD_RESULTS.with_borrow(|seen| assert_eq!(seen, &[false], "SDK negotiation aborts the entered push"));
        STATUSES.with_borrow(|seen| assert!(seen.is_empty(), "no publication reports"));
        assert_eq!(snapshot(dir.path()), before);
        assert_eq!(snapshot(destination.path()), remote_before, "no objects or refs uploaded");
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn push_interrupted_publication_reports_uncertain_effect() {
        let progress = gix::progress::prodash::tree::Root::new();
        let dir = committed_repo();
        let expected = sdk(dir.path()).head_id().unwrap().detach();
        let destination = tempfile::tempdir().unwrap();
        let bare = git2::Repository::init_bare(destination.path().join(".git")).unwrap();
        let before = snapshot(dir.path());
        let interrupt = Arc::new(AtomicBool::new(false));
        let flag = Arc::clone(&interrupt);
        let remote_path = bare.path().to_owned();
        AFTER_UPLOAD.set(Some(Box::new(move || {
            let actual = git2::Repository::open_bare(remote_path).unwrap();
            assert_eq!(actual.refname_to_id("refs/heads/selected").unwrap().as_bytes(), expected.as_slice(),
                "actual remote publication precedes cancellation");
            flag.store(true, Ordering::Relaxed);
        })));
        DESTINATION.set(Some(destination.path().to_owned()));
        QUERIES.set(0);
        WIRES.with_borrow_mut(Vec::clear);
        UPLOAD_RESULTS.with_borrow_mut(Vec::clear);
        let result = ProjectRepo::open(dir.path()).unwrap().push(
            "refs/heads/main", expected, "refs/heads/selected",
            &Url::parse("https://github.com/synthetic/project.git").unwrap(),
            &cade_github::Host::Dotcom, &octocrab::secrecy::SecretString::from("synthetic-token".to_owned()),
            Some(&cade_policy::Policy::default()), progress.add_child("synthetic push"), &interrupt);
        DESTINATION.set(None);
        assert!(result.is_err(), "cancellation cannot report completed success");
        assert_eq!(format!("{}", result.unwrap_err()),
            "git: Git push outcome is uncertain; remote refs may have changed; verify before retrying");
        assert_eq!(QUERIES.get(), 1, "no new connection after cancellation");
        WIRES.with_borrow(|seen| assert_eq!(seen, &[format!("{expected}:refs/heads/selected")], "never automatic retry"));
        UPLOAD_RESULTS.with_borrow(|seen| assert_eq!(seen, &[true]));
        assert_eq!(bare.refname_to_id("refs/heads/selected").unwrap().as_bytes(), expected.as_slice(), "never rollback");
        assert_eq!(bare.references().unwrap().count(), 1);
        assert_eq!(sdk(&destination.path().join(".git")).find_object(expected).unwrap().data,
            sdk(dir.path()).find_object(expected).unwrap().data);
        assert_eq!(snapshot(dir.path()), before, "source guard and local bytes restored");
        let mut messages = Vec::new();
        progress.copy_messages(&mut messages);
        assert!(!messages.is_empty());
        assert!(messages.iter().all(|message| message.level != gix::progress::prodash::messages::MessageLevel::Success));
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn push_already_present_skips_upload() {
        let dir = committed_repo();
        let expected = sdk(dir.path()).head_id().unwrap().detach();
        let destination = tempfile::tempdir().unwrap();
        let bare = git2::Repository::init_bare(destination.path().join(".git")).unwrap();
        let seed = git2::Repository::open(dir.path()).unwrap();
        seed.remote_anonymous(bare.path().to_str().unwrap()).unwrap()
            .push(&[format!("{expected}:refs/heads/selected")], None).unwrap();
        let before = snapshot(dir.path());
        let remote_before = snapshot(destination.path());
        DESTINATION.set(Some(destination.path().to_owned()));
        QUERIES.set(0);
        WIRES.with_borrow_mut(Vec::clear);
        UPLOAD_RESULTS.with_borrow_mut(Vec::clear);
        NEGOTIATIONS.with_borrow_mut(Vec::clear);
        STATUSES.with_borrow_mut(Vec::clear);
        for request in 1..=2 {
            let result = ProjectRepo::open(dir.path()).unwrap().push(
                "refs/heads/main", expected, "refs/heads/selected",
                &Url::parse("https://github.com/synthetic/project.git").unwrap(),
                &cade_github::Host::Dotcom, &octocrab::secrecy::SecretString::from("synthetic-token".to_owned()),
                Some(&cade_policy::Policy::default()), gix::progress::Discard, &AtomicBool::new(false));
            assert_eq!(result.unwrap(), expected);
            assert_eq!(QUERIES.get(), request, "each explicit request independently observes");
            WIRES.with_borrow(|seen| assert!(seen.is_empty(), "zero push calls"));
            UPLOAD_RESULTS.with_borrow(|seen| assert!(seen.is_empty()));
            NEGOTIATIONS.with_borrow(|seen| assert!(seen.is_empty()));
            STATUSES.with_borrow(|seen| assert!(seen.is_empty()));
            assert_eq!(snapshot(dir.path()), before);
            assert_eq!(snapshot(destination.path()), remote_before);
        }
        DESTINATION.set(None);
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn push_fresh_verification_ignores_cached_advertisement() {
        let dir = committed_repo();
        let parent = sdk(dir.path()).head_id().unwrap().detach();
        let expected = advance(dir.path());
        let destination = tempfile::tempdir().unwrap();
        let bare = git2::Repository::init_bare(destination.path().join(".git")).unwrap();
        git(dir.path(), &["update-ref", "refs/remotes/origin/selected", &expected.to_string()]);
        let before = snapshot(dir.path());
        let path = destination.path().join(".git/refs/heads/selected");
        BEFORE_QUERY.set(Some(Box::new(move || {
            let query = QUERIES.get();
            assert_eq!(query, 1);
            BEFORE_QUERY.set(Some(Box::new(move || {
                let query = QUERIES.get();
                assert_eq!(query, 2);
                assert_eq!(fs::read_to_string(&path).unwrap().trim(), expected.to_string(),
                    "actual SDK upload completed before competing remote change");
                fs::write(path, format!("{parent}\n")).unwrap();
            })));
        })));
        DESTINATION.set(Some(destination.path().to_owned()));
        QUERIES.set(0);
        WIRES.with_borrow_mut(Vec::clear);
        UPLOAD_RESULTS.with_borrow_mut(Vec::clear);
        let result = ProjectRepo::open(dir.path()).unwrap().push(
            "refs/heads/main", expected, "refs/heads/selected",
            &Url::parse("https://github.com/synthetic/project.git").unwrap(),
            &cade_github::Host::Dotcom, &octocrab::secrecy::SecretString::from("synthetic-token".to_owned()),
            Some(&cade_policy::Policy::default()), gix::progress::Discard, &AtomicBool::new(false));
        DESTINATION.set(None);
        assert!(result.is_err(), "cached tracking equality must not override fresh observation");
        assert!(format!("{result:?}").contains("explicit non-force retry"));
        assert_eq!(QUERIES.get(), 2);
        WIRES.with_borrow(|seen| assert_eq!(seen, &[format!("{expected}:refs/heads/selected")]));
        UPLOAD_RESULTS.with_borrow(|seen| assert_eq!(seen, &[true]));
        assert_eq!(bare.refname_to_id("refs/heads/selected").unwrap().as_bytes(), parent.as_slice());
        assert!(bare.find_commit(git2::Oid::from_bytes(expected.as_slice()).unwrap()).is_ok(),
            "failed verification does not mean no remote effects");
        assert_eq!(snapshot(dir.path()), before);
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn push_invalid_advertisement_returns_error() {
        use gix::protocol::handshake::Ref;
        for phase in [1, 2] {
            for fault in ["missing", "different", "symbolic", "duplicate", "peeled", "malformed", "rejected-malformed"] {
                // Missing/different pre-query refs permit an ordinary non-force attempt.
                let dir = committed_repo();
                let parent = sdk(dir.path()).head_id().unwrap().detach();
                let expected = advance(dir.path());
                let destination = tempfile::tempdir().unwrap();
                let bare = git2::Repository::init_bare(destination.path().join(".git")).unwrap();
                let before = snapshot(dir.path());
                let remote_before = snapshot(destination.path());
                let name = "refs/heads/selected";
                let direct = Ref::Direct { full_ref_name: name.into(), object: expected };
                let refs = match fault {
                    "missing" => vec![],
                    "different" => vec![Ref::Direct { full_ref_name: name.into(), object: parent }],
                    "symbolic" => vec![Ref::Symbolic { full_ref_name: name.into(), target: "refs/heads/other".into(), tag: None, object: expected }],
                    "duplicate" => vec![direct.clone(), direct],
                    // A peeled tag advertisement is malformed for the selected direct branch.
                    _ => vec![Ref::Peeled { full_ref_name: name.into(), tag: parent, object: expected }],
                };
                QUERY_REFS.set(refs);
                QUERY_OVERRIDE_AT.set(phase);
                if matches!(fault, "malformed" | "rejected-malformed") {
                    // Existing disconnected SDK transport: actual gix rejects an empty/truncated
                    // advertisement. No handwritten receive-pack or fake success protocol.
                    QUERY_DISCONNECTED_AT.set(phase);
                }
                if fault == "rejected-malformed" {
                    STATUS_OVERRIDE.set(true);
                    STATUS_INPUT.set(vec![(name.to_owned(), Some("private-rejection-sentinel".to_owned()))]);
                }
                DESTINATION.set(Some(destination.path().to_owned()));
                QUERIES.set(0);
                WIRES.with_borrow_mut(Vec::clear);
                UPLOAD_RESULTS.with_borrow_mut(Vec::clear);
                let result = ProjectRepo::open(dir.path()).unwrap().push(
                    "refs/heads/main", expected, name,
                    &Url::parse("https://github.com/synthetic/project.git").unwrap(),
                    &cade_github::Host::Dotcom, &octocrab::secrecy::SecretString::from("synthetic-token".to_owned()),
                    Some(&cade_policy::Policy::default()), gix::progress::Discard, &AtomicBool::new(false));
                QUERY_OVERRIDE_AT.set(0);
                QUERY_DISCONNECTED_AT.set(0);
                STATUS_OVERRIDE.set(false);
                DESTINATION.set(None);
                let permits = phase == 1 && matches!(fault, "missing" | "different");
                assert_eq!(result.is_ok(), permits, "phase{phase} {fault}: {result:?}");
                if phase == 2 && fault == "malformed" {
                    assert!(format!("{result:?}").contains("outcome is uncertain; remote refs may have changed"));
                }
                if phase == 2 && fault == "rejected-malformed" {
                    assert!(format!("{result:?}").contains("rejected or incomplete protocol reports"),
                        "strict provider rejection takes precedence over failed observation");
                    assert!(!format!("{result:?}").contains("private-rejection-sentinel"));
                }
                let uploaded = phase == 2 || permits;
                assert_eq!(QUERIES.get(), if uploaded { 2 } else { 1 });
                WIRES.with_borrow(|seen| assert_eq!(seen.len(), usize::from(uploaded)));
                UPLOAD_RESULTS.with_borrow(|seen| assert_eq!(seen, if uploaded { &[true][..] } else { &[][..] }));
                if uploaded {
                    assert_eq!(bare.refname_to_id(name).unwrap().as_bytes(), expected.as_slice(),
                        "observation error must not roll back actual successful upload");
                    assert_eq!(bare.references().unwrap().count(), 1);
                } else {
                    assert_eq!(snapshot(destination.path()), remote_before);
                }
                assert_eq!(snapshot(dir.path()), before);
                println!("phase{phase} {fault}: permitted={permits}, uploaded={uploaded}; state/call counts verified");
            }
        }
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn push_rejected_status_overrides_top_level_success() {
        let dir = committed_repo();
        let expected = sdk(dir.path()).head_id().unwrap().detach();
        let destination = tempfile::tempdir().unwrap();
        let bare = git2::Repository::init_bare(destination.path().join(".git")).unwrap();
        let before = snapshot(dir.path());
        DESTINATION.set(Some(destination.path().to_owned()));
        STATUS_OVERRIDE.set(true);
        STATUS_INPUT.set(vec![("refs/heads/selected".to_owned(), Some("private-rejection-sentinel".to_owned()))]);
        QUERIES.set(0);
        UPLOAD_RESULTS.with_borrow_mut(Vec::clear);
        WIRES.with_borrow_mut(Vec::clear);
        let progress = gix::progress::prodash::tree::Root::new();
        let result = ProjectRepo::open(dir.path()).unwrap().push(
            "refs/heads/main", expected, "refs/heads/selected",
            &Url::parse("https://github.com/synthetic/project.git").unwrap(),
            &cade_github::Host::Dotcom, &octocrab::secrecy::SecretString::from("synthetic-token".to_owned()),
            Some(&cade_policy::Policy::default()), progress.add_child("synthetic push"), &AtomicBool::new(false));
        STATUS_OVERRIDE.set(false);
        DESTINATION.set(None);
        UPLOAD_RESULTS.with_borrow(|seen| assert_eq!(seen, &[true]));
        assert!(result.is_err(), "rejected report overrides top-level Ok AND observed intended tip");
        assert!(!format!("{result:?}").contains("private-rejection-sentinel"));
        let mut messages = Vec::new();
        progress.copy_messages(&mut messages);
        assert!(messages.iter().all(|message| message.level != gix::progress::prodash::messages::MessageLevel::Success),
            "rejection cannot report completed progress");
        assert!(messages.iter().all(|message| !message.message.contains("private-rejection-sentinel")
            && !message.origin.contains("private-rejection-sentinel")));
        assert_eq!(QUERIES.get(), 2);
        WIRES.with_borrow(|seen| assert_eq!(seen, &[format!("{expected}:refs/heads/selected")]));
        assert_eq!(bare.refname_to_id("refs/heads/selected").unwrap().as_bytes(), expected.as_slice());
        assert_eq!(bare.references().unwrap().count(), 1, "no rollback or extra remote ref");
        assert_eq!(snapshot(dir.path()), before);
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn push_invalid_status_reports_return_error() {
        for reports in [
            vec![],
            vec![("refs/heads/selected".to_owned(), None), ("refs/heads/selected".to_owned(), None)],
            vec![("refs/heads/wrong".to_owned(), None)],
        ] {
            let progress = gix::progress::prodash::tree::Root::new();
            let dir = committed_repo();
            let expected = sdk(dir.path()).head_id().unwrap().detach();
            let destination = tempfile::tempdir().unwrap();
            let bare = git2::Repository::init_bare(destination.path().join(".git")).unwrap();
            let before = snapshot(dir.path());
            DESTINATION.set(Some(destination.path().to_owned()));
            STATUS_OVERRIDE.set(true);
            STATUS_INPUT.set(reports.clone());
            QUERIES.set(0);
            UPLOAD_RESULTS.with_borrow_mut(Vec::clear);
            WIRES.with_borrow_mut(Vec::clear);
            let result = ProjectRepo::open(dir.path()).unwrap().push(
                "refs/heads/main", expected, "refs/heads/selected",
                &Url::parse("https://github.com/synthetic/project.git").unwrap(),
                &cade_github::Host::Dotcom, &octocrab::secrecy::SecretString::from("synthetic-token".to_owned()),
                Some(&cade_policy::Policy::default()), progress.add_child("synthetic push"), &AtomicBool::new(false));
            STATUS_OVERRIDE.set(false);
            DESTINATION.set(None);
            UPLOAD_RESULTS.with_borrow(|seen| assert_eq!(seen, &[true]));
            assert!(result.is_err(), "invalid reports {reports:?} must override query equality");
            assert_eq!(QUERIES.get(), 2);
            WIRES.with_borrow(|seen| assert_eq!(seen, &[format!("{expected}:refs/heads/selected")]));
            assert_eq!(bare.refname_to_id("refs/heads/selected").unwrap().as_bytes(), expected.as_slice());
            assert_eq!(bare.references().unwrap().count(), 1);
            assert_eq!(snapshot(dir.path()), before);
            let mut messages = Vec::new();
            progress.copy_messages(&mut messages);
            assert!(!messages.is_empty());
            assert!(messages.iter().all(|message| message.level != gix::progress::prodash::messages::MessageLevel::Success));
            println!("invalid status input {reports:?}: denied despite SDK Ok and observed intended tip; one attempt, local state preserved");
        }
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn push_invalid_negotiation_prevents_upload() {
        for fault in ["missing", "duplicate", "source", "destination", "new-oid", "zero"] {
            let dir = committed_repo();
            let parent = sdk(dir.path()).head_id().unwrap().detach();
            let expected = advance(dir.path());
            let destination = tempfile::tempdir().unwrap();
            let bare = git2::Repository::init_bare(destination.path().join(".git")).unwrap();
            let before = snapshot(dir.path());
            let remote_before = snapshot(destination.path());
            let count = match fault { "missing" => 0, "duplicate" => 2, _ => 1 };
            let source = if fault == "source" { parent } else { expected }.to_string().into_bytes();
            let target = if fault == "destination" { b"refs/heads/wrong".as_slice() } else { b"refs/heads/selected".as_slice() };
            let new = match fault {
                "new-oid" => git2::Oid::from_bytes(parent.as_slice()).unwrap(),
                "zero" => git2::Oid::ZERO_SHA1,
                _ => git2::Oid::from_bytes(expected.as_slice()).unwrap(),
            };
            NEGOTIATION_COUNT.set(Some(count));
            NEGOTIATION_SOURCE.set(source);
            NEGOTIATION_DESTINATION.set(target.to_vec());
            NEGOTIATION_OID.set(Some(new));
            DESTINATION.set(Some(destination.path().to_owned()));
            QUERIES.set(0);
            STATUSES.with_borrow_mut(Vec::clear);
            UPLOAD_RESULTS.with_borrow_mut(Vec::clear);
            WIRES.with_borrow_mut(Vec::clear);
            let result = ProjectRepo::open(dir.path()).unwrap().push(
                "refs/heads/main", expected, "refs/heads/selected",
                &Url::parse("https://github.com/synthetic/project.git").unwrap(),
                &cade_github::Host::Dotcom, &octocrab::secrecy::SecretString::from("synthetic-token".to_owned()),
                Some(&cade_policy::Policy::default()), gix::progress::Discard, &AtomicBool::new(false));
            NEGOTIATION_COUNT.set(None);
            DESTINATION.set(None);
            assert!(result.is_err(), "invalid negotiation {fault} must reject");
            UPLOAD_RESULTS.with_borrow(|seen| assert_eq!(seen, &[false], "SDK must abort before upload: {fault}"));
            STATUSES.with_borrow(|seen| assert!(seen.is_empty(), "no per-ref completion: {fault}"));
            assert_eq!(QUERIES.get(), 2);
            WIRES.with_borrow(|seen| assert_eq!(seen, &[format!("{expected}:refs/heads/selected")]));
            assert_eq!(bare.references().unwrap().count(), 0);
            assert_eq!(snapshot(destination.path()), remote_before, "no objects/refs sent: {fault}");
            assert_eq!(snapshot(dir.path()), before);
            println!("invalid negotiation {fault}: SDK aborted, no objects/refs/statuses sent, local state preserved");
        }
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn push_fast_forward_verifies_remote_commit() {
        existing_destination(false);
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn push_changed_selection_keeps_immutable_wire_oid() {
        existing_destination(true);
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn push_non_fast_forward_preserves_remote_tip() {
        let dir = committed_repo();
        let parent = sdk(dir.path()).head_id().unwrap().detach();
        let divergent = advance(dir.path());
        let destination = tempfile::tempdir().unwrap();
        let bare = git2::Repository::init_bare(destination.path().join(".git")).unwrap();
        let seed = git2::Repository::open(dir.path()).unwrap();
        seed.remote_anonymous(bare.path().to_str().unwrap()).unwrap()
            .push(&[format!("{divergent}:refs/heads/selected")], None).unwrap();
        git(dir.path(), &["reset", "--hard", &parent.to_string()]);
        fs::write(dir.path().join("tracked.txt"), b"local divergent selection\n").unwrap();
        git(dir.path(), &["add", "--", "tracked.txt"]);
        git(dir.path(), &["commit", "-q", "-m", "Local divergent selection"]);
        let expected = sdk(dir.path()).head_id().unwrap().detach();
        assert_ne!(expected, divergent);
        let before = snapshot(dir.path());
        let remote_before = snapshot(destination.path());
        DESTINATION.set(Some(destination.path().to_owned()));
        QUERIES.set(0);
        WIRES.with_borrow_mut(Vec::clear);
        let result = ProjectRepo::open(dir.path()).unwrap().push(
            "refs/heads/main", expected, "refs/heads/selected",
            &Url::parse("https://github.com/synthetic/project.git").unwrap(),
            &cade_github::Host::Dotcom, &octocrab::secrecy::SecretString::from("synthetic-token".to_owned()),
            Some(&cade_policy::Policy::default()), gix::progress::Discard, &AtomicBool::new(false));
        DESTINATION.set(None);
        assert_git_error(result);
        assert_eq!(QUERIES.get(), 2, "fresh observation after rejected actual SDK attempt");
        WIRES.with_borrow(|seen| assert_eq!(seen, &[format!("{expected}:refs/heads/selected")],
            "exactly one nonforce attempt; no deletion or retry"));
        assert_eq!(bare.refname_to_id("refs/heads/selected").unwrap().as_bytes(), divergent.as_slice());
        assert_eq!(snapshot(destination.path()), remote_before);
        assert_eq!(snapshot(dir.path()), before);
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn push_remote_rejection_returns_error() {
        use std::os::unix::fs::PermissionsExt;
        let dir = committed_repo();
        let expected = sdk(dir.path()).head_id().unwrap().detach();
        let destination = tempfile::tempdir().unwrap();
        let bare = git2::Repository::init_bare(destination.path().join(".git")).unwrap();
        let hook = bare.path().join("hooks/pre-receive");
        fs::write(&hook, b"#!/bin/sh\nprintf 'synthetic-rejection-sentinel\\n' >&2\nexit 1\n").unwrap();
        fs::set_permissions(&hook, fs::Permissions::from_mode(0o700)).unwrap();
        let (receive_url, server) = receive_pack(destination.path());
        let before = snapshot(dir.path());
        let progress = gix::progress::prodash::tree::Root::new();
        let captured = Arc::new(std::sync::Mutex::new(Vec::new()));
        let during_upload = Arc::clone(&captured);
        let observer = Arc::clone(&progress);
        AFTER_UPLOAD.set(Some(Box::new(move || observer.sorted_snapshot(&mut during_upload.lock().unwrap()))));
        DESTINATION.set(Some(destination.path().to_owned()));
        RECEIVE_URL.set(Some(receive_url));
        QUERIES.set(0);
        WIRES.with_borrow_mut(Vec::clear);
        STATUSES.with_borrow_mut(Vec::clear);
        let result = ProjectRepo::open(dir.path()).unwrap().push(
            "refs/heads/main", expected, "refs/heads/selected",
            &Url::parse("https://github.com/synthetic/project.git").unwrap(),
            &cade_github::Host::Dotcom, &octocrab::secrecy::SecretString::from("synthetic-token".to_owned()),
            Some(&cade_policy::Policy::default()), progress.add_child("synthetic push"), &AtomicBool::new(false));
        RECEIVE_URL.set(None);
        DESTINATION.set(None);
        let (pid, output) = server.join().unwrap();
        println!("owned localhost receive-pack PID {pid} reaped: {}", output.status);
        assert!(output.status.success(), "receive-pack exchange completed: {output:?}");
        UPLOAD_RESULTS.with_borrow(|seen| assert_eq!(seen, &[true], "actual SDK top-level completion despite rejected ref"));
        assert!(result.is_err(), "actual receive-pack rejection must override transport completion");
        assert!(!format!("{result:?}").contains("synthetic-rejection-sentinel"));
        let tasks = captured.lock().unwrap();
        for name in ["Git push pack objects", "Git push upload objects", "Git push upload bytes"] {
            let task = tasks.iter().find(|(_, task)| task.name == name).unwrap();
            assert!(task.1.progress.as_ref().unwrap().step.load(Ordering::Relaxed) > 0,
                "provider suppression cannot be vacuous absence of numeric progress");
        }
        assert!(tasks.iter().all(|(_, task)| !task.name.contains("synthetic-rejection-sentinel")));
        let mut messages = Vec::new();
        progress.copy_messages(&mut messages);
        assert!(!messages.is_empty(), "fixed client stage messages must remain visible");
        assert!(messages.iter().all(|message| message.level != gix::progress::prodash::messages::MessageLevel::Success));
        assert!(!format!("{messages:?}").contains("synthetic-rejection-sentinel"));
        assert!(!format!("{messages:?}").contains("synthetic-token"));
        assert!(!format!("{messages:?}").contains("https://"));
        assert_eq!(QUERIES.get(), 2);
        WIRES.with_borrow(|seen| assert_eq!(seen, &[format!("{expected}:refs/heads/selected")]));
        STATUSES.with_borrow(|seen| {
            assert_eq!(seen.len(), 1);
            assert_eq!(seen[0].0, "refs/heads/selected");
            assert!(seen[0].1.is_some(), "real rejected per-ref report required");
        });
        assert!(bare.refname_to_id("refs/heads/selected").is_err());
        assert_eq!(bare.references().unwrap().count(), 0);
        assert_eq!(snapshot(dir.path()), before);
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn push_synthetic_diagnostics_preserves_numeric_progress() {
        for sentinel in ["synthetic-header-canary", "synthetic-status-canary", "synthetic-token-canary",
            "synthetic-url-canary", "synthetic-error-canary"] {
            let dir = committed_repo();
            git(dir.path(), &["config", "http.extraHeader", &format!("Authorization: {sentinel}")]);
            let expected = sdk(dir.path()).head_id().unwrap().detach();
            let destination = tempfile::tempdir().unwrap();
            git2::Repository::init_bare(destination.path().join(".git")).unwrap();
            let (receive_url, server) = receive_pack(destination.path());
            RECEIVE_URL.set(Some(receive_url));
            let before = snapshot(dir.path());
            let progress = gix::progress::prodash::tree::Root::new();
            let captured = Arc::new(std::sync::Mutex::new(Vec::new()));
            let during_upload = Arc::clone(&captured);
            let observer = Arc::clone(&progress);
            AFTER_UPLOAD.set(Some(Box::new(move || observer.sorted_snapshot(&mut during_upload.lock().unwrap()))));
            DESTINATION.set(Some(destination.path().to_owned()));
            QUERIES.set(0);
            WIRES.with_borrow_mut(Vec::clear);
            STATUS_OVERRIDE.set(true);
            STATUS_INPUT.set(vec![("refs/heads/selected".to_owned(), Some(format!("Authorization: {sentinel}; provider error")))]);
            let result = ProjectRepo::open(dir.path()).unwrap().push(
                "refs/heads/main", expected, "refs/heads/selected",
                &Url::parse(&format!("https://github.com/synthetic/{sentinel}.git")).unwrap(),
                &cade_github::Host::Dotcom, &octocrab::secrecy::SecretString::from(sentinel.to_owned()),
                Some(&cade_policy::Policy::default()), progress.add_child("synthetic push"), &AtomicBool::new(false));
            DESTINATION.set(None);
            STATUS_OVERRIDE.set(false);
            STATUS_INPUT.with_borrow_mut(Vec::clear);
            RECEIVE_URL.set(None);
            let (_, output) = server.join().unwrap();
            assert!(output.status.success());
            let error = result.unwrap_err();
            assert_eq!(error.to_string(),
                "git: Git push rejected or incomplete protocol reports; inspect remote state before retrying");
            let tasks = captured.lock().unwrap();
            for name in ["Git push pack objects", "Git push upload objects", "Git push upload bytes"] {
                let task = tasks.iter().find(|(_, task)| task.name == name).unwrap();
                assert!(task.1.progress.as_ref().unwrap().step.load(Ordering::Relaxed) > 0);
            }
            let mut messages = Vec::new();
            progress.copy_messages(&mut messages);
            assert!(!messages.is_empty());
            assert!(messages.iter().all(|message| message.level != gix::progress::prodash::messages::MessageLevel::Success));
            for diagnostic in [error.to_string(), format!("{error:?}"), format!("{error:#?}"),
                format!("{messages:?}"), format!("{tasks:?}")] {
                assert!(!diagnostic.contains(sentinel), "synthetic provider material must not escape");
                assert!(!diagnostic.contains("Authorization:"));
                assert!(!diagnostic.contains("https://"));
            }
            assert_eq!(QUERIES.get(), 2);
            assert_eq!(WIRES.with_borrow(Vec::len), 1);
            assert_eq!(snapshot(dir.path()), before);
            println!("diagnostic subcase {sentinel}: fixed error, nonempty messages and positive pack/object/byte counts");
        }
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn push_new_branch_verifies_remote_commit() {
        let dir = committed_repo();
        let parent = sdk(dir.path()).head_id().unwrap().detach();
        advance(dir.path());
        let destination = tempfile::tempdir().unwrap();
        let bare = git2::Repository::init_bare(destination.path().join(".git")).unwrap();
        let repo = ProjectRepo::open(dir.path()).unwrap();
        let source = sdk(dir.path());
        let expected = source.head_id().unwrap().detach();
        let before = snapshot(dir.path());
        let remote = Url::parse("https://github.com/synthetic/project.git").unwrap();
        let token = octocrab::secrecy::SecretString::from("synthetic-token".to_owned());
        let policy = cade_policy::Policy::default();
        DESTINATION.set(Some(destination.path().to_owned()));
        QUERIES.set(0);
        NEGOTIATIONS.with_borrow_mut(Vec::clear);
        let result = repo.push("refs/heads/main", expected, "refs/heads/selected",
            &remote, &cade_github::Host::Dotcom, &token, Some(&policy),
            gix::progress::Discard, &AtomicBool::new(false));
        DESTINATION.set(None);
        assert!(result.is_ok(), "actual owned bare upload must verify the selected commit: {result:?}");
        assert_eq!(result.unwrap(), expected);
        assert_eq!(QUERIES.get(), 2, "independent pre/post gix observations required");
        NEGOTIATIONS.with_borrow(|seen| {
            assert_eq!(seen.len(), 1);
            assert!(seen[0].0.is_zero(), "SDK src is the missing remote old OID");
            assert_eq!(seen[0].1.as_bytes(), expected.as_slice(), "SDK dst is the proposed new OID");
            assert_eq!(seen[0].2, expected.to_string().as_bytes(), "full immutable OID source");
        });
        assert_eq!(bare.refname_to_id("refs/heads/selected").unwrap().as_bytes(), expected.as_slice());
        assert_eq!(bare.references().unwrap().count(), 1);
        let observed = sdk(&destination.path().join(".git"));
        let commit = source.find_commit(expected).unwrap();
        assert_eq!(observed.find_object(expected).unwrap().data, commit.data);
        assert_eq!(commit.parent_ids().map(|id| id.detach()).collect::<Vec<_>>(), vec![parent]);
        assert_eq!(observed.find_object(parent).unwrap().data, source.find_object(parent).unwrap().data);
        let tree = commit.tree().unwrap();
        assert_eq!(observed.find_object(tree.id).unwrap().data, tree.data);
        for entry in tree.iter() {
            let id = entry.unwrap().oid().to_owned();
            assert_eq!(observed.find_object(id).unwrap().data, source.find_object(id).unwrap().data);
        }
        assert_eq!(snapshot(dir.path()), before);
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn git_push_remote_options_preserves_mechanics_callbacks() {
        run_isolated(|| {
        use std::os::unix::fs::PermissionsExt;
        let dir = committed_repo();
        let destination = tempfile::tempdir().unwrap();
        let bare = git2::Repository::init_bare(destination.path().join(".git")).unwrap();
        let hook = bare.path().join("hooks/pre-receive");
        fs::write(&hook, b"#!/bin/sh\nprintf 'synthetic-sideband-sentinel\\n' >&2\nexit 0\n").unwrap();
        fs::set_permissions(&hook, fs::Permissions::from_mode(0o700)).unwrap();
        let (receive_url, server) = receive_pack(destination.path());
        let repository = git2::Repository::open(dir.path()).unwrap();
        let expected = repository.refname_to_id("refs/heads/main").unwrap();
        let before = snapshot(dir.path());
        let remote = Url::parse("https://github.com/synthetic/project.git").unwrap();
        let token = octocrab::secrecy::SecretString::from("synthetic-token".to_owned());
        let policy = cade_policy::Policy::default();
        let interrupt = AtomicBool::new(false);
        let mut negotiations = 0;
        let mut statuses = 0;
        let mut progress = 0;
        let mut packed = 0;
        let mut transferred = 0;
        let mut callbacks = git2::RemoteCallbacks::new();
        callbacks.push_negotiation(|updates| {
            negotiations += 1;
            assert_eq!(updates.len(), 1);
            assert_eq!(updates[0].dst(), expected);
            Ok(())
        });
        callbacks.push_update_reference(|name, status| {
            assert_eq!(name, "refs/heads/selected");
            assert!(status.is_none());
            statuses += 1;
            Ok(())
        });
        callbacks.pack_progress(|_, current, _| packed = current);
        callbacks.push_transfer_progress(|_, _, bytes| {
            progress += 1;
            transferred = bytes;
        });
        callbacks.sideband_progress(|_| panic!("incoming provider text callback must be replaced"));
        callbacks.credentials(|_, _, _| panic!("incoming credentials must never be used"));
        let (_selected, mut options) = cade_github::Host::Dotcom.git_push_remote(
            &repository, &remote, &token, Some(&policy), callbacks, &interrupt).unwrap();
        // Approved local SDK seam: reuse the actual factory options unchanged.
        // This exercises mechanics retention, not HTTPS authentication or TLS.
        let mut local = repository.remote_anonymous(&receive_url).unwrap();
        local.push(&[format!("{expected}:refs/heads/selected")], Some(&mut options)).unwrap();
        let (_, output) = server.join().unwrap();
        assert!(output.status.success());
        drop(options);
        assert_eq!(negotiations, 1);
        assert_eq!(statuses, 1);
        assert!(progress > 0);
        assert!(packed > 0);
        assert!(transferred > 0);
        assert_eq!(bare.refname_to_id("refs/heads/selected").unwrap(), expected);
        assert_eq!(snapshot(dir.path()), before);
        let canceled_destination = tempfile::tempdir().unwrap();
        let canceled = git2::Repository::init_bare(canceled_destination.path()).unwrap();
        let canceled_before = snapshot(canceled_destination.path());
        let mut entered = 0;
        let mut canceled_statuses = 0;
        let mut callbacks = git2::RemoteCallbacks::new();
        callbacks.push_negotiation(|_| {
            entered += 1;
            interrupt.store(true, Ordering::Relaxed);
            if interrupt.load(Ordering::Relaxed) {
                return Err(git2::Error::from_str("Git push interrupted"));
            }
            Ok(())
        });
        callbacks.push_update_reference(|_, _| {
            canceled_statuses += 1;
            Ok(())
        });
        let (_, mut options) = cade_github::Host::Dotcom.git_push_remote(
            &repository, &remote, &token, Some(&policy), callbacks, &interrupt).unwrap();
        let mut local = repository.remote_anonymous(canceled_destination.path().to_str().unwrap()).unwrap();
        assert!(local.push(&[format!("{expected}:refs/heads/selected")], Some(&mut options)).is_err());
        drop(options);
        assert_eq!(entered, 1, "original cancellation callback must remain installed");
        assert_eq!(canceled_statuses, 0);
        assert!(interrupt.load(Ordering::Relaxed));
        assert!(canceled.refname_to_id("refs/heads/selected").is_err());
        assert_eq!(snapshot(canceled_destination.path()), canceled_before);
        assert_eq!(snapshot(dir.path()), before);
        });
    }

    #[test]
    fn git_push_remote_denied_request_preserves_configuration() {
        run_isolated(|| {
        let dir = committed_repo();
        git(dir.path(), &["config", "http.proxy", "http://proxy.example.invalid:9"]);
        let repository = git2::Repository::open(dir.path()).unwrap();
        let before = snapshot(dir.path());
        let remote = Url::parse("https://github.com/synthetic/project.git").unwrap();
        let token = octocrab::secrecy::SecretString::from("synthetic-token".to_owned());
        let interrupt = AtomicBool::new(false);
        let result = cade_github::Host::Dotcom.git_push_remote(
            &repository, &remote, &token, None, git2::RemoteCallbacks::new(), &interrupt);
        assert!(result.is_err(), "missing policy must deny before changing private configuration");
        assert_eq!(repository.config().unwrap().get_string("http.proxy").unwrap(), "http://proxy.example.invalid:9");
        assert_eq!(snapshot(dir.path()), before);
        let allow = cade_policy::Policy::default();
        let mut off = cade_policy::Policy::default();
        off.tools.network = cade_policy::Switch::Off;
        let empty = octocrab::secrecy::SecretString::from(String::new());
        let unselected = Url::parse("https://unselected.example.invalid/synthetic/project.git").unwrap();
        for (case, selected, credential, policy) in [
            ("None", &remote, &token, None),
            ("Off", &remote, &token, Some(&off)),
            ("empty-token", &remote, &empty, Some(&allow)),
            ("unselected-host", &unselected, &token, Some(&allow)),
        ] {
            for attempt in 0..2 {
                let result = cade_github::Host::Dotcom.git_push_remote(
                    &repository, selected, credential, policy, git2::RemoteCallbacks::new(), &interrupt);
                assert!(result.is_err(), "{case} attempt{attempt} must deny");
                assert_eq!(repository.config().unwrap().get_string("http.proxy").unwrap(), "http://proxy.example.invalid:9");
                assert_eq!(snapshot(dir.path()), before, "{case} attempt{attempt} preserves persistent configuration");
            }
        }
        });
    }

    #[test]
    fn git_push_remote_precanceled_request_preserves_configuration() {
        let dir = committed_repo();
        git(dir.path(), &["config", "http.proxy", "http://proxy.example.invalid:9"]);
        let repository = git2::Repository::open(dir.path()).unwrap();
        let before = snapshot(dir.path());
        let remote = Url::parse("https://github.com/synthetic/project.git").unwrap();
        let token = octocrab::secrecy::SecretString::from("synthetic-token".to_owned());
        let policy = cade_policy::Policy::default();
        let interrupt = AtomicBool::new(true);
        let result = cade_github::Host::Dotcom.git_push_remote(
            &repository, &remote, &token, Some(&policy), git2::RemoteCallbacks::new(), &interrupt);
        assert!(result.is_err(), "original interruption must deny before changing private configuration");
        assert_eq!(repository.config().unwrap().get_string("http.proxy").unwrap(), "http://proxy.example.invalid:9");
        assert_eq!(snapshot(dir.path()), before);
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn git_push_remote_poisoned_config_keeps_explicit_destination() {
        run_isolated(|| {
        let dir = committed_repo();
        git(dir.path(), &["config", "url.https://other.example.invalid/.insteadOf", "https://github.com/"]);
        git(dir.path(), &["config", "url.https://push.example.invalid/.pushInsteadOf", "https://github.com/"]);
        git(dir.path(), &["config", "http.proxy", "http://proxy.example.invalid:9"]);
        let marker = dir.path().join("helper-executed");
        let helper = format!("!touch '{}'", marker.display());
        for (key, value) in [
            ("remote.origin.url", "https://other.example.invalid/synthetic/project.git"),
            ("remote.origin.pushurl", "https://push.example.invalid/synthetic/project.git"),
            ("remote.origin.push", "+refs/heads/*:refs/heads/*"),
            ("remote.origin.fetch", "+refs/heads/*:refs/remotes/origin/*"),
            ("remote.origin.proxy", "http://proxy.example.invalid:9"),
            ("http.extraHeader", "Authorization: synthetic-header-sentinel"),
            ("http.followRedirects", "true"),
            ("http.sslVerify", "false"),
            ("credential.helper", helper.as_str()),
            ("core.askPass", helper.as_str()),
            ("gitoxide.tracePacket", "true"),
            ("http.verbose", "true"),
        ] {
            git(dir.path(), &["config", key, value]);
        }
        let before = snapshot(dir.path());
        let repository = git2::Repository::open(dir.path()).unwrap();
        let remote = Url::parse("https://github.com/synthetic/project.git").unwrap();
        let token = octocrab::secrecy::SecretString::from("synthetic-token".to_owned());
        let policy = cade_policy::Policy::default();
        let interrupt = AtomicBool::new(false);
        let result = cade_github::Host::Dotcom.git_push_remote(
            &repository, &remote, &token, Some(&policy), git2::RemoteCallbacks::new(), &interrupt);
        assert!(result.is_ok(), "valid lazy push factory must construct despite poisoned repository config");
        let (actual, _options) = result.unwrap();
        assert_eq!(actual.url().unwrap(), remote.as_str());
        assert!(actual.pushurl().unwrap().is_none_or(|url| url == remote.as_str()));
        assert!(repository.config().unwrap().get_string("http.proxy").is_err());
        assert_eq!(snapshot(dir.path()), before);
        assert!(repository.config().unwrap().entries(None).unwrap().next().is_none(), "operation-private config must be empty");
        assert_eq!(actual.refspecs().count(), 0, "configured refspecs cannot reach anonymous remote");
        assert!(!marker.exists(), "factory must not invoke helper or askpass");
        let destination = tempfile::tempdir().unwrap();
        let bare = git2::Repository::init_bare(destination.path()).unwrap();
        let expected = repository.refname_to_id("refs/heads/main").unwrap();
        let mut options = _options;
        let mut local = repository.remote_anonymous(destination.path().to_str().unwrap()).unwrap();
        local.push(&[format!("{expected}:refs/heads/selected")], Some(&mut options)).unwrap();
        assert_eq!(bare.refname_to_id("refs/heads/selected").unwrap(), expected);
        assert_eq!(bare.references().unwrap().count(), 1);
        assert!(!marker.exists(), "actual local SDK upload must not invoke helper or askpass");
        assert_eq!(snapshot(dir.path()), before);
        // Local SDK transport proves config/helper isolation, not on-wire HTTPS,
        // redirect, proxy, native trust, or process trace isolation.
        });
    }

    #[test]
    fn push_corrupt_object_prevents_transport() {
        let dir = committed_repo();
        let repo = ProjectRepo::open(dir.path()).unwrap();
        let expected = sdk(dir.path()).head_id().unwrap().detach();
        let hex = expected.to_string();
        let object = dir.path().join(".git/objects").join(&hex[..2]).join(&hex[2..]);
        let saved = fs::read(&object).unwrap();
        // Existing Git fixtures replace immutable loose files rather than
        // assuming their permissions allow an in-place overwrite.
        fs::remove_file(&object).unwrap();
        fs::write(&object, b"corrupt loose object storage").unwrap();
        let before = snapshot(dir.path());
        let entered = std::cell::Cell::new(0);
        let result = repo.push_with_source("refs/heads/main", expected, || {}, |_| {
            entered.set(entered.get() + 1);
            Ok(expected)
        });
        assert!(matches!(result, Err(GitError::Git(_))), "corrupt selected object must fail: {result:?}");
        assert_eq!(entered.get(), 0, "corrupt storage must not enter transport");
        assert_eq!(snapshot(dir.path()), before);
        fs::write(&object, saved).unwrap();
        let repaired = snapshot(dir.path());
        assert_eq!(repo.push_with_source("refs/heads/main", expected, || {}, |_| Ok(expected)).unwrap(), expected);
        assert_eq!(snapshot(dir.path()), repaired);
    }

    #[test]
    fn push_symbolic_source_after_lock_prevents_upload() {
        let dir = committed_repo();
        let repo = ProjectRepo::open(dir.path()).unwrap();
        let expected = sdk(dir.path()).head_id().unwrap().detach();
        let reference = dir.path().join(".git/refs/heads/main");
        fs::write(&reference, b"ref: refs/heads/topic\n").unwrap();
        let changed = snapshot(dir.path());
        fs::write(&reference, format!("{expected}\n")).unwrap();
        let entered = std::cell::Cell::new(0);
        let result = repo.push_with_source("refs/heads/main", expected, || {
            assert!(dir.path().join(".git/refs/heads/main.lock").exists());
            fs::write(&reference, b"ref: refs/heads/topic\n").unwrap();
        }, |_| {
            entered.set(entered.get() + 1);
            Ok(expected)
        });
        assert!(matches!(result, Err(GitError::Git(_))), "symbolic source under lock must fail: {result:?}");
        assert_eq!(entered.get(), 0);
        assert_eq!(snapshot(dir.path()), changed);
    }

    #[test]
    fn push_changed_source_after_lock_prevents_upload() {
        let dir = committed_repo();
        let repo = ProjectRepo::open(dir.path()).unwrap();
        let expected = sdk(dir.path()).head_id().unwrap().detach();
        let reference = dir.path().join(".git/refs/heads/main");
        git(dir.path(), &["commit", "--allow-empty", "-q", "-m", "Other selection"]);
        let changed = fs::read(&reference).unwrap();
        fs::write(&reference, format!("{expected}\n")).unwrap();
        let mut expected_snapshot = snapshot(dir.path());
        fs::write(&reference, &changed).unwrap();
        let changed_snapshot = snapshot(dir.path());
        fs::write(&reference, format!("{expected}\n")).unwrap();
        assert_eq!(snapshot(dir.path()), expected_snapshot);
        let entered = std::cell::Cell::new(0);
        let result = repo.push_with_source("refs/heads/main", expected, || {
            assert!(dir.path().join(".git/refs/heads/main.lock").exists());
            // An intentionally noncooperating fixture writer; no SDK locks or sleeps.
            fs::write(&reference, &changed).unwrap();
        }, |_| {
            entered.set(entered.get() + 1);
            Ok(expected)
        });
        assert!(matches!(result, Err(GitError::Git(_))), "changed source under lock must fail: {result:?}");
        assert_eq!(entered.get(), 0, "locked revalidation must precede upload");
        assert_eq!(snapshot(dir.path()), changed_snapshot, "only fixture writer changes may remain");
        fs::write(&reference, format!("{expected}\n")).unwrap();
        expected_snapshot = snapshot(dir.path());
        assert_eq!(repo.push_with_source("refs/heads/main", expected, || {}, |_| Ok(expected)).unwrap(), expected);
        assert_eq!(snapshot(dir.path()), expected_snapshot);
    }

    #[test]
    fn push_namespace_prevents_transport() {
        let dir = committed_repo();
        let repo = ProjectRepo::open(dir.path()).unwrap();
        let expected = sdk(dir.path()).head_id().unwrap().detach();
        git(dir.path(), &["update-ref", "refs/namespaces/selected/refs/heads/main", &expected.to_string()]);
        git(dir.path(), &["config", "gitoxide.core.refsNamespace", "selected"]);
        assert!(sdk(dir.path()).namespace().is_some(), "fixture must activate the SDK namespace");
        let before = snapshot(dir.path());
        let entered = std::cell::Cell::new(0);
        let result = repo.push_with_source("refs/heads/main", expected, || {}, |_| {
            entered.set(entered.get() + 1);
            Ok(expected)
        });
        assert_eq!(snapshot(dir.path()), before);
        assert_eq!(QUERIES.get(), 0);
        assert!(WIRES.with_borrow(Vec::is_empty));
        assert!(matches!(result, Err(GitError::Git(_))), "namespace must deny before transport: {result:?}");
        assert_eq!(entered.get(), 0, "namespace must not enter query/upload composition");
    }

    #[test]
    fn push_executable_filter_prevents_transport() {
        let mut denied = Vec::new();
        for key in ["filter.synthetic.clean", "filter.synthetic.process", "filter.synthetic.smudge"] {
            let dir = committed_repo();
            let repo = ProjectRepo::open(dir.path()).unwrap();
            let expected = sdk(dir.path()).head_id().unwrap().detach();
            let marker = dir.path().join("filter-executed");
            git(dir.path(), &["config", key, &format!("touch '{}'", marker.display())]);
            let before = snapshot(dir.path());
            let entered = std::cell::Cell::new(0);
            let result = repo.push_with_source("refs/heads/main", expected, || {}, |_| {
                entered.set(entered.get() + 1);
                Ok(expected)
            });
            assert_eq!(snapshot(dir.path()), before, "{key}");
            assert!(!marker.exists(), "{key} must never execute");
            assert_eq!(QUERIES.get(), 0, "{key}");
            assert!(WIRES.with_borrow(Vec::is_empty), "{key}");
            let rejected = matches!(result, Err(GitError::Git(_))) && entered.get() == 0;
            println!("filter subcase {key}: denied={rejected}, composition_entries={}", entered.get());
            denied.push((key, rejected));
        }
        assert!(denied.iter().all(|(_, rejected)| *rejected), "every executable filter must deny before composition: {denied:?}");
    }

    #[test]
    fn push_unsupported_repository_prevents_transport() {
        for (case, settings) in [
            ("future-version", vec![("core.repositoryFormatVersion", "999")]),
            ("sha256", vec![("core.repositoryFormatVersion", "1"), ("extensions.objectFormat", "sha256")]),
            ("unknown-hash", vec![("core.repositoryFormatVersion", "1"), ("extensions.objectFormat", "unknown")]),
        ] {
            let dir = committed_repo();
            let repo = ProjectRepo::open(dir.path()).unwrap();
            let expected = sdk(dir.path()).head_id().unwrap().detach();
            for (key, value) in settings {
                git(dir.path(), &["config", key, value]);
            }
            println!("unsupported fixture setup complete: {case}; existing SHA1 objects retained deliberately");
            let before = snapshot(dir.path());
            let entered = std::cell::Cell::new(0);
            let result = repo.push_with_source("refs/heads/main", expected, || {}, |_| {
                entered.set(entered.get() + 1);
                Ok(expected)
            });
            assert!(matches!(result, Err(GitError::Git(_))), "{case} must deny: {result:?}");
            assert_eq!(entered.get(), 0, "{case}");
            assert_eq!(QUERIES.get(), 0, "{case}");
            assert!(WIRES.with_borrow(Vec::is_empty), "{case}");
            assert_eq!(snapshot(dir.path()), before, "{case}");
            println!("unsupported fixture {case}: denied before transport, snapshot unchanged");
        }
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn push_required_objects_prevents_transport() {
        let mut denied = Vec::new();
        for selected in ["parent", "root-tree", "nested-tree", "blob"] {
            for fault in ["missing", "corrupt", "wrong-bytes"] {
                let dir = committed_repo();
                fs::create_dir(dir.path().join("nested")).unwrap();
                fs::write(dir.path().join("nested/history"), b"required historical blob").unwrap();
                git(dir.path(), &["add", "nested"]);
                git(dir.path(), &["commit", "-q", "-m", "Required ancestry"]);
                let expression = match selected {
                    "parent" => "HEAD",
                    "root-tree" => "HEAD^{tree}",
                    "nested-tree" => "HEAD:nested",
                    "blob" => "HEAD:nested/history",
                    _ => unreachable!(),
                };
                let object_id = String::from_utf8(git(dir.path(), &["rev-parse", expression])).unwrap();
                let object_id = object_id.trim();
                git(dir.path(), &["rm", "-qr", "nested"]);
                git(dir.path(), &["commit", "-q", "-m", "Selected tip"]);
                let expected = sdk(dir.path()).head_id().unwrap().detach();
                let repo = ProjectRepo::open(dir.path()).unwrap();
                let object = dir.path().join(".git/objects").join(&object_id[..2]).join(&object_id[2..]);
                match fault {
                    "missing" => fs::remove_file(&object).unwrap(),
                    "corrupt" => {
                        fs::remove_file(&object).unwrap();
                        fs::write(&object, b"not compressed Git data").unwrap();
                    }
                    "wrong-bytes" => {
                        let other = sdk(dir.path()).write_blob(b"valid compressed object with wrong identity").unwrap().detach().to_string();
                        fs::remove_file(&object).unwrap();
                        fs::copy(dir.path().join(".git/objects").join(&other[..2]).join(&other[2..]), &object).unwrap();
                    }
                    _ => unreachable!(),
                }
                println!("required-object fixture setup complete: {selected}/{fault}");
                let destination = tempfile::tempdir().unwrap();
                git2::Repository::init_bare(destination.path().join(".git")).unwrap();
                let before = snapshot(dir.path());
                let remote_before = snapshot(destination.path());
                DESTINATION.set(Some(destination.path().to_owned()));
                QUERIES.set(0);
                WIRES.with_borrow_mut(Vec::clear);
                let result = repo.push(
                    "refs/heads/main", expected, "refs/heads/selected",
                    &Url::parse("https://github.com/synthetic/project.git").unwrap(),
                    &cade_github::Host::Dotcom,
                    &octocrab::secrecy::SecretString::from("synthetic-token".to_owned()),
                    Some(&cade_policy::Policy::default()), gix::progress::Discard, &AtomicBool::new(false),
                );
                DESTINATION.set(None);
                let rejected = matches!(result, Err(GitError::Git(_))) && QUERIES.get() == 0 && WIRES.with_borrow(Vec::is_empty);
                println!("required-object {selected}/{fault}: denied_before_transport={rejected}, queries={}, uploads={}", QUERIES.get(), WIRES.with_borrow(Vec::len));
                assert_eq!(snapshot(dir.path()), before, "{selected}/{fault}");
                // A red may enter upload; preserve its actual remote side effects in
                // output rather than confuse them with fixture construction.
                println!("required-object {selected}/{fault}: remote_unchanged={}", snapshot(destination.path()) == remote_before);
                denied.push((selected, fault, rejected));
            }
        }
        assert!(denied.iter().all(|(_, _, rejected)| *rejected), "required objects must deny before query/upload: {denied:?}");
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn push_wrong_kind_required_objects_prevents_transport() {
        let mut denied = Vec::new();
        for selected in ["blob-as-tree", "blob-as-commit", "parent-as-blob", "root-as-blob", "nested-as-blob"] {
            let dir = committed_repo();
            let source = sdk(dir.path());
            let original = source.head_id().unwrap().detach();
            let mut commit = source.find_commit(original).unwrap().decode().unwrap().to_owned().unwrap();
            let blob = source.write_blob(b"synthetic valid object").unwrap().detach();
            let wrong = match selected {
                "blob-as-tree" => commit.tree,
                "blob-as-commit" => original,
                _ => blob,
            };
            let object = source.find_object(wrong).unwrap();
            let data = gix::objs::Data::new(&object.data, object.kind, wrong.kind());
            data.verify_checksum(&wrong).unwrap();
            data.decode().unwrap();
            match selected {
                "parent-as-blob" => commit.parents = vec![wrong].into(),
                "root-as-blob" => commit.tree = wrong,
                _ => {
                    commit.tree = source.write_object(&gix::objs::Tree {
                        entries: vec![gix::objs::tree::Entry {
                            mode: if selected == "nested-as-blob" {
                                gix::objs::tree::EntryKind::Tree.into()
                            } else {
                                gix::objs::tree::EntryKind::Blob.into()
                            },
                            filename: "wrong-kind".into(),
                            oid: wrong,
                        }],
                    }).unwrap().detach();
                }
            }
            let expected = source.write_object(&commit).unwrap().detach();
            fs::write(dir.path().join(".git/refs/heads/main"), format!("{expected}\n")).unwrap();
            let destination = tempfile::tempdir().unwrap();
            git2::Repository::init_bare(destination.path().join(".git")).unwrap();
            let before = snapshot(dir.path());
            let remote_before = snapshot(destination.path());
            DESTINATION.set(Some(destination.path().to_owned()));
            QUERIES.set(0);
            WIRES.with_borrow_mut(Vec::clear);
            let result = ProjectRepo::open(dir.path()).unwrap().push(
                "refs/heads/main", expected, "refs/heads/selected",
                &Url::parse("https://github.com/synthetic/project.git").unwrap(),
                &cade_github::Host::Dotcom,
                &octocrab::secrecy::SecretString::from("synthetic-token".to_owned()),
                Some(&cade_policy::Policy::default()), gix::progress::Discard, &AtomicBool::new(false),
            );
            DESTINATION.set(None);
            let rejected = matches!(result, Err(GitError::Git(_))) && QUERIES.get() == 0 && WIRES.with_borrow(Vec::is_empty);
            println!("valid-OID wrong-kind {selected}: denied={rejected}, queries={}, uploads={}, remote_unchanged={}",
                QUERIES.get(), WIRES.with_borrow(Vec::len), snapshot(destination.path()) == remote_before);
            assert_eq!(snapshot(dir.path()), before, "{selected}");
            denied.push((selected, rejected, snapshot(destination.path()) == remote_before));
        }
        assert!(denied.iter().all(|(_, rejected, unchanged)| *rejected && *unchanged),
            "valid-OID wrong-kind required objects must deny before query/upload: {denied:?}");
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn push_gitlink_and_lfs_pointer_preserves_opaque_objects() {
        let dir = committed_repo();
        let missing = "1234567890123456789012345678901234567890";
        git(dir.path(), &["update-index", "--add", "--cacheinfo", &format!("160000,{missing},external")]);
        fs::write(dir.path().join(".gitattributes"), b"payload filter=lfs\n").unwrap();
        fs::write(dir.path().join("payload"), b"version https://git-lfs.github.com/spec/v1\noid sha256:0000000000000000000000000000000000000000000000000000000000000000\nsize 99\n").unwrap();
        git(dir.path(), &["add", ".gitattributes", "payload"]);
        git(dir.path(), &["commit", "-q", "-m", "Opaque references"]);
        let expected = sdk(dir.path()).head_id().unwrap().detach();
        assert_eq!(git2::Oid::from_bytes(expected.as_slice()).unwrap().as_bytes(), expected.as_slice(), "lossless pinned SDK identifier conversion");
        let destination = assert_push_preserves_repository(dir.path());
        let observed = sdk(&destination.path().join(".git"));
        assert!(!observed.has_object(gix::ObjectId::from_hex(missing.as_bytes()).unwrap()), "gitlink target must not be traversed");
        assert!(!destination.path().join(".git/lfs").exists(), "no LFS payload traversal");
    }

    #[test]
    fn push_malformed_commit_prevents_transport() {
        let dir = committed_repo();
        let repo = ProjectRepo::open(dir.path()).unwrap();
        let fixture = dir.path().join("malformed-commit-fixture");
        fs::write(&fixture, b"not a decodable commit\n").unwrap();
        let output = git(dir.path(), &["hash-object", "--literally", "-t", "commit", "-w", "malformed-commit-fixture"]);
        fs::remove_file(fixture).unwrap();
        let expected = gix::ObjectId::from_hex(String::from_utf8(output).unwrap().trim().as_bytes()).unwrap();
        fs::write(dir.path().join(".git/refs/heads/main"), format!("{expected}\n")).unwrap();
        let before = snapshot(dir.path());
        let entered = std::cell::Cell::new(0);
        let result = repo.push_with_source("refs/heads/main", expected, || {}, |_| {
            entered.set(entered.get() + 1);
            Ok(expected)
        });
        assert!(matches!(result, Err(GitError::Git(_))), "undecodable selected commit must fail: {result:?}");
        assert_eq!(entered.get(), 0, "malformed commit must not enter transport");
        assert_eq!(snapshot(dir.path()), before);
    }

    #[test]
    fn push_noncommit_object_prevents_transport() {
        let dir = committed_repo();
        let repo = ProjectRepo::open(dir.path()).unwrap();
        let expected = sdk(dir.path()).write_blob(b"not a commit").unwrap().detach();
        // Raw ref bytes are intentional: fixture setup must not reject the
        // noncommit selection before the push validator can observe it.
        fs::write(dir.path().join(".git/refs/heads/main"), format!("{expected}\n")).unwrap();
        let before = snapshot(dir.path());
        let entered = std::cell::Cell::new(0);
        let result = repo.push_with_source("refs/heads/main", expected, || {}, |_| {
            entered.set(entered.get() + 1);
            Ok(expected)
        });
        assert!(matches!(result, Err(GitError::Git(_))), "noncommit selected object must fail: {result:?}");
        assert_eq!(entered.get(), 0, "noncommit object must not enter transport");
        assert_eq!(snapshot(dir.path()), before);
    }

    #[test]
    fn push_invalid_object_prevents_transport() {
        let dir = committed_repo();
        let repo = ProjectRepo::open(dir.path()).unwrap();
        let expected = sdk(dir.path()).head_id().unwrap().detach();
        let hex = expected.to_string();
        let object = dir.path().join(".git/objects").join(&hex[..2]).join(&hex[2..]);
        let saved = fs::read(&object).unwrap();
        fs::remove_file(&object).unwrap();
        let before = snapshot(dir.path());
        let entered = std::cell::Cell::new(0);
        let result = repo.push_with_source("refs/heads/main", expected, || {}, |_| {
            entered.set(entered.get() + 1);
            Ok(expected)
        });
        assert!(matches!(result, Err(GitError::Git(_))), "missing selected object must fail: {result:?}");
        assert_eq!(entered.get(), 0, "missing object must not enter transport");
        assert_eq!(snapshot(dir.path()), before);
        fs::write(&object, saved).unwrap();
        let repaired = snapshot(dir.path());
        assert_eq!(repo.push_with_source("refs/heads/main", expected, || {}, |_| Ok(expected)).unwrap(), expected);
        assert_eq!(snapshot(dir.path()), repaired);
    }

    #[test]
    fn push_symbolic_source_prevents_transport() {
        let dir = committed_repo();
        let repo = ProjectRepo::open(dir.path()).unwrap();
        let expected = sdk(dir.path()).head_id().unwrap().detach();
        git(dir.path(), &["symbolic-ref", "refs/heads/topic", "refs/heads/main"]);
        let before = snapshot(dir.path());
        let entered = std::cell::Cell::new(0);
        let result = repo.push_with_source("refs/heads/topic", expected, || {}, |_| {
            entered.set(entered.get() + 1);
            Ok(expected)
        });
        assert!(matches!(result, Err(GitError::Git(_))), "symbolic selection must fail: {result:?}");
        assert_eq!(entered.get(), 0);
        assert_eq!(snapshot(dir.path()), before);
    }

    #[test]
    fn push_missing_source_prevents_transport() {
        let dir = committed_repo();
        let repo = ProjectRepo::open(dir.path()).unwrap();
        let expected = sdk(dir.path()).head_id().unwrap().detach();
        let before = snapshot(dir.path());
        let entered = std::cell::Cell::new(0);
        let result = repo.push_with_source("refs/heads/missing", expected, || {}, |_| {
            entered.set(entered.get() + 1);
            Ok(expected)
        });
        assert!(matches!(result, Err(GitError::Git(_))), "missing selection must fail: {result:?}");
        assert_eq!(entered.get(), 0);
        assert_eq!(snapshot(dir.path()), before);
    }

    #[test]
    fn push_locked_source_preserves_foreign_lock() {
        let dir = committed_repo();
        let repo = ProjectRepo::open(dir.path()).unwrap();
        let expected = sdk(dir.path()).head_id().unwrap().detach();
        let marker = dir.path().join(".git/refs/heads/main.lock");
        fs::write(&marker, b"foreign marker").unwrap();
        let before = snapshot(dir.path());
        let entered = std::cell::Cell::new(0);
        let result = repo.push_with_source("refs/heads/main", expected, || {}, |_| {
            entered.set(entered.get() + 1);
            Ok(expected)
        });
        assert!(matches!(result, Err(GitError::Git(_))), "foreign lock must obstruct push: {result:?}");
        assert_eq!(entered.get(), 0);
        assert_eq!(snapshot(dir.path()), before);
        fs::remove_file(marker).unwrap();
        let corrected = snapshot(dir.path());
        let result = repo.push_with_source("refs/heads/main", expected, || {}, |_| Ok(expected));
        assert_eq!(result.unwrap(), expected, "explicitly corrected local preparation may proceed");
        assert_eq!(snapshot(dir.path()), corrected);
        #[cfg(any(target_os = "macos", target_os = "linux"))]
        {
            let destination = tempfile::tempdir().unwrap();
            let bare = git2::Repository::init_bare(destination.path().join(".git")).unwrap();
            DESTINATION.set(Some(destination.path().to_owned()));
            QUERIES.set(0);
            WIRES.with_borrow_mut(Vec::clear);
            let result = repo.push("refs/heads/main", expected, "refs/heads/selected",
                &Url::parse("https://github.com/synthetic/project.git").unwrap(),
                &cade_github::Host::Dotcom, &octocrab::secrecy::SecretString::from("synthetic-token".to_owned()),
                Some(&cade_policy::Policy::default()), gix::progress::Discard, &AtomicBool::new(false));
            DESTINATION.set(None);
            assert_eq!(result.unwrap(), expected, "corrected explicit retry completes actual SDK publication");
            assert_eq!(QUERIES.get(), 2);
            WIRES.with_borrow(|seen| assert_eq!(seen, &[format!("{expected}:refs/heads/selected")]));
            assert_eq!(bare.refname_to_id("refs/heads/selected").unwrap().as_bytes(), expected.as_slice());
            assert_eq!(snapshot(dir.path()), corrected);
        }
    }

    #[test]
    fn push_competing_writer_retains_selected_tip() {
        let dir = committed_repo();
        let repo = ProjectRepo::open(dir.path()).unwrap();
        let expected = sdk(dir.path()).head_id().unwrap().detach();
        let before = snapshot(dir.path());
        let result = repo.push_with_source("refs/heads/main", expected, || {}, |_| {
            let competing = git2::Repository::open(dir.path()).unwrap();
            let mut writer = competing.transaction().unwrap();
            assert!(writer.lock_ref("refs/heads/main").is_err(), "selected source must stay locked during the operation");
            assert!(!dir.path().join(".git/HEAD.lock").exists(), "push must not lock HEAD");
            Ok(expected)
        });
        assert_eq!(result.unwrap(), expected);
        assert_eq!(snapshot(dir.path()), before, "no-edit guard must leave every local entry unchanged");
        let competing = git2::Repository::open(dir.path()).unwrap();
        let mut writer = competing.transaction().unwrap();
        assert!(writer.lock_ref("refs/heads/main").is_ok(), "guard drop must release its own marker");
        drop(writer);
        assert_eq!(snapshot(dir.path()), before);
    }

    #[test]
    fn push_stale_tip_prevents_transport() {
        let dir = committed_repo();
        let repo = ProjectRepo::open(dir.path()).unwrap();
        let expected = sdk(dir.path()).head_id().unwrap().detach();
        git(dir.path(), &["commit", "--allow-empty", "-q", "-m", "New selection"]);
        let before = snapshot(dir.path());
        let entered = std::cell::Cell::new(0);
        let result = repo.push_with_source("refs/heads/main", expected, || {}, |_| {
            entered.set(entered.get() + 1);
            Ok(expected)
        });
        assert!(matches!(result, Err(GitError::Git(_))), "stale selection must fail: {result:?}");
        assert_eq!(entered.get(), 0, "stale selection must not enter transport composition");
        assert_eq!(snapshot(dir.path()), before);
    }
    }
}

fn remote() -> Url {
    Url::parse("https://forge.example.invalid/synthetic/project.git").unwrap()
}

fn sdk(path: &Path) -> gix::Repository {
    gix::open_opts(path, gix::open::Options::isolated().strict_config(true)).unwrap()
}

fn actor() -> gix::actor::SignatureRef<'static> {
    gix::actor::SignatureRef::from_bytes(b"Transport Fixture <fixture@example.invalid> 946684800 +0000").unwrap()
}

// A reader, not a transport/client framework. Cancellation is triggered by an
// actual server advertisement read, not by a timer or a pre-canceled token.
struct InterruptRead<R> {
    inner: R,
    interrupt: Arc<AtomicBool>,
}

impl<R: Read> Read for InterruptRead<R> {
    fn read(&mut self, bytes: &mut [u8]) -> std::io::Result<usize> {
        let count = self.inner.read(bytes)?;
        if count != 0 {
            self.interrupt.store(true, Ordering::SeqCst);
        }
        Ok(count)
    }
}

pub(super) fn with_upload_pack<R>(
    source: &Path,
    interrupt_on_read: Option<Arc<AtomicBool>>,
    operation: impl FnOnce(Box<dyn Transport + Send>) -> R,
) -> (R, std::process::Output) {
    with_upload_pack_io(
        source,
        |_| {},
        |stdout| match interrupt_on_read {
            Some(interrupt) => Box::new(InterruptRead { inner: stdout, interrupt }),
            None => Box::new(stdout),
        },
        operation,
    )
}

// Keep server isolation, SDK connection construction and panic-safe reaping in
// one fixture. Callers may configure only the owned server and observe its pipe.
fn with_upload_pack_io<R>(
    source: &Path,
    configure: impl FnOnce(&mut Command),
    observe: impl FnOnce(std::process::ChildStdout) -> Box<dyn Read + Send>,
    operation: impl FnOnce(Box<dyn Transport + Send>) -> R,
) -> (R, std::process::Output) {
    // Git is the SERVER only. The client speaks real V2 through the pinned SDK
    // over owned pipes. No git clone/fetch, generic connector, DNS, TLS or socket.
    // Every source fixture is a worktree. --strict disables the worktree-to-.git
    // fallback, so name its metadata directory explicitly.
    let mut command = Command::new("git");
    command
        .env_clear()
        .envs(std::env::vars_os().filter(|(key, _)| matches!(key.to_str(), Some("PATH" | "SystemRoot" | "SYSTEMROOT" | "TEMP" | "TMP"))))
        .args(["-c", "uploadpack.allowFilter=false"])
        .current_dir(source)
        .env("GIT_CONFIG_GLOBAL", if cfg!(windows) { "NUL" } else { "/dev/null" })
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_ATTR_NOSYSTEM", "1")
        .env("GIT_PROTOCOL", "version=2")
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("HOME", source)
        .env("USERPROFILE", source)
        .env("XDG_CONFIG_HOME", source)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    configure(&mut command);
    let mut child = command
        .args(["upload-pack", "--strict", ".git"])
        .spawn()
        .expect("fixture setup requires Git with protocol V2 upload-pack");
    let read = observe(child.stdout.take().unwrap());
    let transport = gix::protocol::transport::client::git::blocking_io::Connection::new(
        read,
        child.stdin.take().unwrap(),
        Protocol::V2,
        "/synthetic/project.git",
        None::<(String, Option<u16>)>,
        ConnectMode::Process,
        false,
    )
    .custom_url(Some(remote().as_str().into()));
    // Reap the server even if a product assertion panics. Dropping the client
    // closes its pipes; upload-pack observes EOF. The executor supplies a timeout.
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| operation(Box::new(transport))));
    let output = child.wait_with_output().expect("reap owned upload-pack fixture");
    match result {
        Ok(value) => (value, output),
        Err(panic) => std::panic::resume_unwind(panic),
    }
}

pub(super) fn disconnected() -> Box<dyn Transport + Send> {
    Box::new(gix::protocol::transport::client::git::blocking_io::Connection::new(
        std::io::Cursor::new(Vec::<u8>::new()),
        std::io::sink(),
        Protocol::V2,
        "/synthetic/project.git",
        None::<(String, Option<u16>)>,
        ConnectMode::Process,
        false,
    ))
}

fn fetch(repo: &ProjectRepo, source: &Path) -> Result<gix::remote::fetch::Outcome, GitError> {
    with_upload_pack(source, None, |transport| {
        repo.fetch_with_transport(&remote(), transport, gix::progress::Discard, &AtomicBool::new(false))
    })
    .0
}

fn advance(source: &Path) -> gix::ObjectId {
    fs::write(source.join("tracked.txt"), b"remote revision\n").unwrap();
    git(source, &["add", "--", "tracked.txt"]);
    git(source, &["commit", "-q", "-m", "Remote revision"]);
    sdk(source).head_id().unwrap().detach()
}

fn selection(path: &Path) -> std::collections::BTreeMap<std::path::PathBuf, (fs::FileType, Vec<u8>)> {
    let mut state = snapshot(path);
    state.retain(|path, _| {
        !path.starts_with(".git/objects")
            && !path.starts_with(".git/refs/remotes")
            && !path.starts_with(".git/logs/refs/remotes")
            && path != Path::new(".git/FETCH_HEAD")
    });
    state
}

fn assert_received(destination: &Path, source: &Path, reference: &str) {
    let expected = sdk(source).head_id().unwrap().detach();
    let local = sdk(destination);
    let actual = local.find_reference(reference).unwrap().try_id().unwrap().detach();
    assert_eq!(actual, expected);
    assert_eq!(
        local.find_object(expected).unwrap().data,
        sdk(source).find_object(expected).unwrap().data
    );
    let blob = git(source, &["rev-parse", "HEAD:tracked.txt"]);
    let blob = gix::ObjectId::from_hex(blob.trim_ascii()).unwrap();
    assert_eq!(local.find_object(blob).unwrap().data, b"remote revision\n");
}
