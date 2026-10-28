//! Shared owned-repository fixtures for integration and private-boundary probes.

use super::{GitError, ProjectRepo};
use std::path::Path;
use std::process::Command;

// Keep ordinary libtest names, attributes and bodies in both consumers. Only
// the selected child runs a fixture; the parallel parent never touches SDK state.
macro_rules! isolated_tests {
    () => {};
    ($(#[cfg($cfg:meta)])? #[test] fn $name:ident() $body:block $($rest:tt)*) => {
        $(#[cfg($cfg)])?
        #[test]
        fn $name() {
            run_isolated(|| $body);
        }
        isolated_tests! { $($rest)* }
    };
    ($item:item $($rest:tt)*) => {
        $item
        isolated_tests! { $($rest)* }
    };
}

pub(super) fn run_isolated(test: impl FnOnce()) {
    let thread = std::thread::current();
    let name = thread.name().expect("libtest names each test thread");
    if std::env::var("CADE_GIT_TEST_CHILD").as_deref() == Ok(name) {
        assert_eq!(
            std::env::args_os().skip(1).collect::<Vec<_>>(),
            ["--exact", name, "--nocapture", "--test-threads=1"].map(std::ffi::OsString::from),
            "SDK initialization requires exactly one selected test and one test thread"
        );
        let config = tempfile::tempdir().unwrap();
        for level in [
            git2::ConfigLevel::System,
            git2::ConfigLevel::Global,
            git2::ConfigLevel::XDG,
            git2::ConfigLevel::ProgramData,
        ] {
            // SAFETY: the marker AND exact argv invariant above restrict this
            // branch to one selected libtest in a one-test-thread subprocess.
            // The parallel parent never enters it. All callers invoke this
            // before fixture/configuration objects or fixture threads exist.
            // The regression's reentry also precedes its fixtures and drops
            // its diagnostic Config first. No reader can race these writes.
            // Freeze all paths before test() starts; never append defaults or
            // reset paths afterward. Process exit, not a reset, ends isolation.
            unsafe { git2::opts::set_search_path(level, config.path()) }.unwrap();
        }
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
        .args(["--exact", name, "--nocapture", "--test-threads=1"])
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
fn run_isolated_child_without_single_test_arguments_is_rejected() {
    let thread = std::thread::current();
    let name = thread.name().unwrap();
    if std::env::var("CADE_GIT_TEST_CHILD").as_deref() == Ok(name) {
        run_isolated(|| {});
        return;
    }
    let out = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", name, "--nocapture"])
        .env("CADE_GIT_TEST_CHILD", name)
        .output()
        .unwrap();
    assert!(!out.status.success(), "a child marker alone must not authorize SDK global access");
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("SDK initialization requires exactly one selected test and one test thread"),
        "the selection invariant must fail before SDK access: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn run_isolated_sdk_search_paths_do_not_reach_fixture() {
    run_isolated(|| {
        let hostile = tempfile::tempdir().unwrap();
        for file in ["gitconfig", ".gitconfig", "config"] {
            std::fs::write(hostile.path().join(file), b"[filter \"owned-sdk\"]\n\tclean = false\n").unwrap();
        }
        for level in [
            git2::ConfigLevel::System,
            git2::ConfigLevel::Global,
            git2::ConfigLevel::XDG,
            git2::ConfigLevel::ProgramData,
        ] {
            // SAFETY: run_isolated executes this closure only in its exact,
            // one-test, one-thread child. No SDK objects or fixture threads
            // exist yet. All mutations precede configuration/fixture reads;
            // no reset occurs after fixture users start.
            unsafe { git2::opts::set_search_path(level, hostile.path()) }.unwrap();
        }
        // Prove the owned SDK path really supplies a filter, independently of
        // gix and GIT_CONFIG_*; drop the config before reinitializing isolation.
        {
            let config = git2::Config::open_default().unwrap();
            assert_eq!(config.get_string("filter.owned-sdk.clean").unwrap(), "false");
        }
        run_isolated(|| {
            let dir = committed_repo();
            git(dir.path(), &["branch", "topic"]);
            let repo = ProjectRepo::open(dir.path()).unwrap();
            let actor = gix::actor::SignatureRef::from_bytes(b"Checkout Actor <checkout@example.invalid> 946688400 +0530").unwrap();
            let result = repo.checkout("refs/heads/topic", actor);
            assert!(result.is_ok(), "owned SDK configuration must not reach checkout: {result:?}");
        });
    });
}

#[test]
fn run_isolated_sdk_search_paths_replace_every_level() {
    run_isolated(|| {
        let hostile = tempfile::tempdir().unwrap();
        let levels = [
            git2::ConfigLevel::System,
            git2::ConfigLevel::Global,
            git2::ConfigLevel::XDG,
            git2::ConfigLevel::ProgramData,
        ];
        for level in levels {
            // SAFETY: the exact one-test/one-thread child has no configuration
            // objects, concurrent SDK readers, or spawned fixture threads.
            unsafe { git2::opts::set_search_path(level, hostile.path()) }.unwrap();
        }
        run_isolated(|| {
            let mut paths = Vec::new();
            for level in levels {
                // SAFETY: still before any fixture/SDK object or thread exists,
                // in the same selected child. Initialization has completed and
                // there are no concurrent SDK readers or writers.
                let path = unsafe { git2::opts::get_search_path(level) }.unwrap();
                let path = Path::new(path.to_str().unwrap());
                assert_ne!(path, hostile.path(), "{level:?} must replace, not append the hostile path");
                assert!(path.is_dir(), "{level:?} must use one owned directory, not a search list");
                assert_eq!(std::fs::read_dir(path).unwrap().count(), 0, "{level:?} must be empty");
                paths.push(path.to_path_buf());
            }
            assert!(
                paths.windows(2).all(|pair| pair[0] == pair[1]),
                "all four levels must use the same owned directory"
            );
        });
    });
}

#[test]
fn git_executable_search_path_reaches_fixture() {
    let thread = std::thread::current();
    let name = thread.name().unwrap();
    if std::env::var("CADE_GIT_TEST_CHILD").as_deref() == Ok(name) {
        run_isolated(|| {
            let dir = tempfile::tempdir().unwrap();
            std::fs::create_dir(dir.path().join("cade-fixture-path-canary")).unwrap();
            // Git's test-only shell alias reports the Git child's PATH, not the
            // Rust parent's. A relative owned entry also survives Git for
            // Windows' shell path conversion without assuming a drive spelling.
            let path = git(dir.path(), &["-c", "alias.fixture-path=!printf '%s' \"$PATH\"", "fixture-path"]);
            assert!(
                String::from_utf8_lossy(&path).contains("cade-fixture-path-canary"),
                "the Git fixture must retain the caller's executable search PATH: {}",
                String::from_utf8_lossy(&path)
            );
        });
        return;
    }
    let path = std::env::var_os("PATH").expect("the test caller supplies an executable search PATH");
    let path =
        std::env::join_paths(std::iter::once(std::path::PathBuf::from("cade-fixture-path-canary")).chain(std::env::split_paths(&path)))
            .unwrap();
    let out = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", name, "--nocapture", "--test-threads=1"])
        .env("CADE_GIT_TEST_CHILD", name)
        .env("PATH", path)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "Git fixture PATH propagation failed:\n{}\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(String::from_utf8_lossy(&out.stdout).contains("1 passed; 0 failed"));
}

pub(super) fn git(dir: &Path, args: &[&str]) -> Vec<u8> {
    let out = Command::new("git")
        .env_clear()
        .envs(
            ["PATH", "SystemRoot", "SYSTEMROOT", "TEMP", "TMP"]
                .into_iter()
                .filter_map(|key| std::env::var_os(key).map(|value| (key, value))),
        )
        .args([
            "-c",
            "user.name=CADE Test",
            "-c",
            "user.email=test@example.invalid",
            "-c",
            "commit.gpgsign=false",
            "-c",
            "init.templateDir=",
            "-c",
            "core.autocrlf=false",
            "-c",
            "maintenance.auto=false",
        ])
        .args(args)
        .current_dir(dir)
        .env("GIT_CONFIG_GLOBAL", if cfg!(windows) { "NUL" } else { "/dev/null" })
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_ATTR_NOSYSTEM", "1")
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("HOME", dir)
        .env("USERPROFILE", dir)
        .env("XDG_CONFIG_HOME", dir)
        .env("GIT_AUTHOR_DATE", "2000-01-01T00:00:00+0000")
        .env("GIT_COMMITTER_DATE", "2000-01-01T00:00:00+0000")
        .output()
        .expect("git is installed");
    assert!(out.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
    out.stdout
}

pub(super) fn committed_repo() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q", "-b", "main"]);
    std::fs::write(dir.path().join("tracked.txt"), b"original\n").unwrap();
    git(dir.path(), &["add", "--", "tracked.txt"]);
    git(dir.path(), &["commit", "-q", "-m", "Fixture"]);
    dir
}

// Include empty directories and symlink targets without following symlinks.
// Comparing the whole fixture also catches lock files, config/credential writes,
// changed objects/refs, and index stat-cache writes.
pub(super) fn snapshot(root: &Path) -> std::collections::BTreeMap<std::path::PathBuf, (std::fs::FileType, Vec<u8>)> {
    fn visit(root: &Path, dir: &Path, entries: &mut std::collections::BTreeMap<std::path::PathBuf, (std::fs::FileType, Vec<u8>)>) {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            let kind = std::fs::symlink_metadata(&path).unwrap().file_type();
            let bytes = if kind.is_symlink() {
                std::fs::read_link(&path).unwrap().as_os_str().as_encoded_bytes().to_vec()
            } else if kind.is_file() {
                std::fs::read(&path).unwrap()
            } else {
                Vec::new()
            };
            entries.insert(path.strip_prefix(root).unwrap().to_path_buf(), (kind, bytes));
            if kind.is_dir() {
                visit(root, &path, entries);
            }
        }
    }
    let mut entries = std::collections::BTreeMap::new();
    visit(root, root, &mut entries);
    entries
}

pub(super) fn assert_git_error<T>(result: Result<T, GitError>) -> String {
    match result {
        Err(GitError::Git(message)) => {
            assert!(!message.trim().is_empty(), "Git errors must be actionable");
            message
        }
        Err(other) => panic!("expected Git error, got {other}"),
        Ok(_) => panic!("expected Git error, got success"),
    }
}
