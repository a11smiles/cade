//! Integration tests for `cade_git::ProjectRepo` against real repositories in
//! temp dirs. Fixtures are built with the `git` CLI with global and system
//! config disabled, so the developer's `~/.gitconfig` is never read.

use cade_git::{GitError, ProjectRepo, run_commit_message};
use std::process::Command;

// Keep the ordinary libtest names, attributes and bodies. Each body runs in a
// fresh process, since changing process-wide Git environment in parallel tests
// would race and isolating only the fixture's Git CLI does not isolate gix.
macro_rules! isolated_tests {
    () => {};
    ($(#[cfg($cfg:meta)])? #[test] fn $name:ident() $body:block $($rest:tt)*) => {
        $(#[cfg($cfg)])?
        #[test]
        fn $name() {
            crate::run_isolated(|| $body);
        }
        isolated_tests! { $($rest)* }
    };
    ($item:item $($rest:tt)*) => {
        $item
        isolated_tests! { $($rest)* }
    };
}

#[path = "repo/branches.rs"]
mod branches;
#[path = "repo/checkout.rs"]
mod checkout;
#[path = "repo/commit.rs"]
mod commit;
#[path = "repo/local.rs"]
mod local;
#[path = "repo/merge_base.rs"]
mod merge_base;
#[path = "repo/objects.rs"]
mod objects;
#[path = "repo/status.rs"]
mod status;
#[path = "repo/support.rs"]
mod support;

use support::{assert_git_error, committed_repo, git, snapshot};

fn run_isolated(test: impl FnOnce()) {
    let thread = std::thread::current();
    let name = thread.name().expect("libtest names each test thread");
    if std::env::var("CADE_GIT_TEST_CHILD").as_deref() == Ok(name) {
        test();
        return;
    }

    let home = tempfile::tempdir().unwrap();
    let config = home.path().join("empty-config");
    std::fs::write(&config, b"").unwrap();
    let out = Command::new(std::env::current_exe().unwrap())
        .env_clear()
        .envs(
            // Named lookup honors Windows' case-insensitive environment while
            // emitting canonical keys for the isolated fixture's Git helper.
            ["PATH", "SystemRoot", "SYSTEMROOT", "COPILOT_CLI_EXTRACT_DIR"]
                .into_iter()
                .filter_map(|key| std::env::var_os(key).map(|value| (key, value))),
        )
        .args(["--exact", name, "--nocapture"])
        .env("CADE_GIT_TEST_CHILD", name)
        .env("HOME", home.path())
        .env("USERPROFILE", home.path())
        .env("XDG_CONFIG_HOME", home.path())
        .env("TMPDIR", home.path())
        .env("TEMP", home.path())
        .env("TMP", home.path())
        .env("GIT_CONFIG_GLOBAL", &config)
        .env("GIT_CONFIG_SYSTEM", &config)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_ATTR_NOSYSTEM", "1")
        .env("GIT_TERMINAL_PROMPT", "0")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "isolated test {name} failed:\n{}\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        String::from_utf8_lossy(&out.stdout).contains("1 passed; 0 failed"),
        "the isolated child must execute exactly the selected test: {}",
        String::from_utf8_lossy(&out.stdout)
    );
}

#[test]
fn run_isolated_failed_child_is_reported() {
    let thread = std::thread::current();
    let name = thread.name().unwrap();
    if std::env::var("CADE_GIT_TEST_CHILD").as_deref() == Ok(name) {
        panic!("intentional child assertion failure");
    }
    let result = std::panic::catch_unwind(|| run_isolated(|| {}));
    assert!(result.is_err(), "a failing child must fail its parent test");
}

#[test]
fn run_isolated_executable_search_path_reaches_child() {
    run_isolated(|| {
        let path = std::env::var_os("PATH").expect("the isolated child must retain the executable search PATH");
        assert!(!path.is_empty(), "the executable search PATH must not be empty");
    });
}

#[test]
fn run_isolated_ambient_configuration_does_not_reach_fixture() {
    let home = tempfile::tempdir().unwrap();
    let config = home.path().join(".gitconfig");
    std::fs::write(&config, b"[filter \"ambient\"]\n\tclean = false\n\tprocess = false\n").unwrap();
    let out = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "status::status_clean_repository_returns_no_events", "--nocapture"])
        .env("HOME", home.path())
        .env("USERPROFILE", home.path())
        .env("XDG_CONFIG_HOME", home.path())
        .env("GIT_CONFIG_GLOBAL", &config)
        .env("GIT_CONFIG_SYSTEM", &config)
        .env("GIT_CONFIG_NOSYSTEM", "0")
        .env("GIT_CONFIG_COUNT", "1")
        .env("GIT_CONFIG_KEY_0", "filter.command.clean")
        .env("GIT_CONFIG_VALUE_0", "false")
        .env_remove("CADE_GIT_TEST_CHILD")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "ambient configuration reached the fixture:\n{}\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(String::from_utf8_lossy(&out.stdout).contains("1 passed; 0 failed"));
}

isolated_tests! {
#[test]
fn git_fixture_disables_automatic_maintenance() {
    let dir = committed_repo();
    // Force a deterministic maintenance task, in the foreground so the oracle
    // does not depend on winning the snapshot/background-writer race from CI.
    git(dir.path(), &["config", "maintenance.auto", "true"]);
    git(dir.path(), &["config", "maintenance.autoDetach", "false"]);
    git(dir.path(), &["config", "maintenance.loose-objects.enabled", "true"]);
    git(dir.path(), &["config", "maintenance.loose-objects.auto", "1"]);
    let before = snapshot(&dir.path().join(".git/objects/pack"));
    git(dir.path(), &["commit", "-q", "--allow-empty", "-m", "No fixture maintenance"]);
    assert_eq!(snapshot(&dir.path().join(".git/objects/pack")), before);
    assert_eq!(
        git(dir.path(), &["config", "--type=bool", "--default=true", "--get", "maintenance.auto"]),
        b"false\n",
        "fixture commands must not launch background writers after returning"
    );
}

#[test]
fn project_repo_open_plain_directory_returns_not_a_repo_with_path() {
    let dir = tempfile::tempdir().unwrap();
    match ProjectRepo::open(dir.path()) {
        Err(GitError::NotARepo(p)) => assert_eq!(p, dir.path()),
        other => panic!("expected NotARepo, got {:?}", other.err()),
    }
}

#[test]
fn project_repo_open_missing_directory_returns_not_a_repo() {
    let dir = tempfile::tempdir().unwrap();
    let missing = dir.path().join("does-not-exist");
    assert!(matches!(ProjectRepo::open(&missing), Err(GitError::NotARepo(p)) if p == missing));
}

#[test]
fn project_repo_init_new_directory_has_workdir_and_no_commits() {
    let dir = tempfile::tempdir().unwrap();
    let r = ProjectRepo::init(dir.path()).unwrap();
    assert!(!r.has_commits());
    assert_eq!(r.workdir().unwrap().canonicalize().unwrap(), dir.path().canonicalize().unwrap());
    assert!(dir.path().join(".git").is_dir());
}

#[test]
fn project_repo_init_existing_repo_returns_git_error() {
    let dir = tempfile::tempdir().unwrap();
    ProjectRepo::init(dir.path()).unwrap();
    let before = snapshot(dir.path());
    assert!(matches!(ProjectRepo::init(dir.path()), Err(GitError::Git(_))));
    assert_eq!(snapshot(dir.path()), before);
}

#[test]
fn project_repo_branch_named_branch_returns_short_name() {
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q", "-b", "cade/intake-flow"]);
    let r = ProjectRepo::open(dir.path()).unwrap();
    assert_eq!(r.branch().unwrap().as_deref(), Some("cade/intake-flow"));
}

#[test]
fn project_repo_has_commits_after_first_commit_returns_true() {
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q", "-b", "main"]);
    git(dir.path(), &["commit", "-q", "--allow-empty", "-m", "Save"]);
    assert!(ProjectRepo::open(dir.path()).unwrap().has_commits());
}

#[test]
fn project_repo_branch_detached_head_returns_none() {
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q", "-b", "main"]);
    git(dir.path(), &["commit", "-q", "--allow-empty", "-m", "Save"]);
    git(dir.path(), &["checkout", "-q", "--detach"]);
    let r = ProjectRepo::open(dir.path()).unwrap();
    assert_eq!(r.branch().unwrap(), None);
    assert!(r.has_commits());
}

#[test]
fn project_repo_open_bare_repo_has_no_workdir() {
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q", "--bare", "-b", "main"]);
    assert!(ProjectRepo::open(dir.path()).unwrap().workdir().is_none());
}

#[test]
fn run_commit_message_committed_with_git_is_parsed_as_trailers() {
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q", "-b", "main"]);
    let msg = run_commit_message("Built Step 1", "run-42", "gpt-5", 0);
    git(dir.path(), &["commit", "-q", "--allow-empty", "-m", &msg]);
    let out = Command::new("git")
        .args(["log", "-1", "--format=%(trailers:only,unfold)"])
        .current_dir(dir.path())
        .env("GIT_CONFIG_GLOBAL", if cfg!(windows) { "NUL" } else { "/dev/null" })
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .output()
        .unwrap();
    assert_eq!(
        String::from_utf8(out.stdout).unwrap().trim_end(),
        "CADE-Copilot-Run: run-42\nCADE-Model: gpt-5\nCADE-Tokens: 0"
    );
}
}
