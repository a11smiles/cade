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

fn with_upload_pack<R>(
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

fn disconnected() -> Box<dyn Transport + Send> {
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
