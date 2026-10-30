//! Read-only drift checks. Generate explicitly; tests must never repair source.

#[path = "../examples/support/mod.rs"]
mod support;

fn tracked(path: &str) -> String {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    // An absent artifact must be an assertion failure, not a compilation failure.
    std::fs::read_to_string(root.join(path)).unwrap_or_default()
}

#[test]
fn bindings_checked_in_artifact_matches_rust_declarations() {
    assert_eq!(
        tracked("app/ui/src/ipc/bindings.ts"),
        support::bindings(),
        "run cargo run -p cade-feedback --example generate_feedback -- bindings and apply stdout"
    );
}

#[test]
fn bindings_native_commands_projects_only_caller_arguments() {
    assert!(
        support::bindings().contains(
            "export type Commands = {\n\
             current_policy: { args: undefined; result: JsonValue; error: CommandError };\n\
             check_shell: { args: { command: string; }; result: string; error: CommandError };\n\
             preview_feedback: { args: { feedback: Feedback; }; result: IssueDraft; error: CommandError };\n\
             submit_feedback: { args: { feedback: Feedback; }; result: string; error: CommandError };\n\
             };\n"
        ),
        "bindings must project all four native signatures, excluding injected State"
    );
}

#[test]
fn bindings_command_error_preserves_message_envelope() {
    assert!(
        support::bindings().contains("export type CommandError = { \"Message\": string };"),
        "bindings must declare the existing externally tagged Message error"
    );
}

#[test]
fn forms_checked_in_artifact_matches_hosted_yaml() {
    assert_eq!(
        tracked("app/ui/src/feedback/forms.generated.ts"),
        support::forms().expect("hosted YAML generates form metadata"),
        "run cargo run -p cade-feedback --example generate_feedback -- forms and apply stdout"
    );
}

#[test]
fn bindings_and_forms_autocrlf_checkout_preserves_generated_bytes() {
    let fixture = tempfile::tempdir().expect("create owned checkout fixture");
    let repo = fixture.path().join("repo");
    let home = fixture.path().join("home");
    let empty_config = fixture.path().join("empty-config");
    std::fs::create_dir(&repo).expect("create fixture repository directory");
    std::fs::create_dir(&home).expect("create isolated fixture home");
    std::fs::write(&empty_config, b"").expect("create empty Git configuration and attributes file");

    // Only this child process's environment changes. No ambient repository,
    // config, attributes, credentials or templates may influence the fixture.
    let git = |args: &[&str]| {
        let mut command = std::process::Command::new("git");
        command.env_clear();
        // Windows needs its executable search path, system root and temp paths.
        for key in ["PATH", "SystemRoot", "TEMP", "TMP"] {
            if let Some(value) = std::env::var_os(key) {
                command.env(key, value);
            }
        }
        let output = command
            .current_dir(&repo)
            .env("HOME", &home)
            .env("USERPROFILE", &home)
            .env("XDG_CONFIG_HOME", &home)
            .env("GIT_CONFIG_GLOBAL", &empty_config)
            .env("GIT_CONFIG_SYSTEM", &empty_config)
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_ATTR_NOSYSTEM", "1")
            .env("GIT_TERMINAL_PROMPT", "0")
            .env("GIT_ALLOW_PROTOCOL", "")
            .args([
                "--no-pager",
                "-c",
                "core.attributesFile=../empty-config",
                "-c",
                "core.excludesFile=../empty-config",
                "-c",
                "core.fsmonitor=false",
                "-c",
                "core.safecrlf=false",
                "-c",
                "credential.helper=",
            ])
            .args(args)
            .output()
            .expect("run installed Git for checkout fixture");
        assert!(
            output.status.success(),
            "fixture git {args:?} failed ({}): {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
        output.stdout
    };

    git(&["init", "--template=", "--initial-branch=main", "."]);
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    // Exercise production rules, not a duplicated or repaired attribute fixture.
    std::fs::copy(root.join(".gitattributes"), repo.join(".gitattributes")).expect("copy real source attributes into fixture");
    git(&["-c", "core.autocrlf=false", "add", "--", ".gitattributes"]);

    let artifacts = [
        ("app/ui/src/ipc/bindings.ts", support::bindings().into_bytes()),
        (
            "app/ui/src/feedback/forms.generated.ts",
            support::forms().expect("hosted YAML generates form metadata").into_bytes(),
        ),
        (
            "app/ui/public/fonts/OFL.txt",
            std::fs::read(root.join("app/ui/public/fonts/OFL.txt")).expect("read canonical font license"),
        ),
        (
            "app/ui/public/fonts/PROVENANCE.md",
            std::fs::read(root.join("app/ui/public/fonts/PROVENANCE.md")).expect("read canonical font provenance"),
        ),
    ];
    let control_path = "unprotected-control.txt";
    let control_lf = b"first line\nsecond line\n";
    for (path, bytes) in artifacts
        .iter()
        .map(|(path, bytes)| (*path, bytes.as_slice()))
        .chain(std::iter::once((control_path, control_lf.as_slice())))
    {
        assert!(
            bytes.contains(&b'\n') && !bytes.contains(&b'\r'),
            "{path}: fixture input must be canonical LF"
        );
        let file = repo.join(path);
        std::fs::create_dir_all(file.parent().expect("fixture file has a parent")).expect("create fixture artifact directories");
        std::fs::write(&file, bytes).expect("write generated bytes or synthetic control into fixture");
        git(&["-c", "core.autocrlf=false", "add", "--", path]);
        assert_eq!(
            git(&["show", &format!(":{path}")]),
            bytes,
            "{path}: index blob must retain canonical bytes"
        );
    }

    // Force fresh materialization: matching cached stat data must not let Git
    // retain the LF files staged above. Remove only these fixture-owned files.
    for path in artifacts.iter().map(|(path, _)| *path).chain(std::iter::once(control_path)) {
        std::fs::remove_file(repo.join(path)).expect("remove fixture working file before checkout");
    }
    git(&[
        "-c",
        "core.autocrlf=true",
        "checkout-index",
        "--",
        artifacts[0].0,
        artifacts[1].0,
        artifacts[2].0,
        artifacts[3].0,
        control_path,
    ]);
    let control = std::fs::read(repo.join(control_path)).expect("read checked-out control");
    assert_eq!(
        control, b"first line\r\nsecond line\r\n",
        "unprotected control must prove checkout conversion is active"
    );
    eprintln!(
        "checkout control passed: {} LF bytes became {} CRLF bytes",
        control_lf.len(),
        control.len()
    );

    let mut mismatches = Vec::new();
    for (path, expected) in &artifacts {
        let actual = std::fs::read(repo.join(path)).expect("read checked-out generated artifact");
        if actual != *expected {
            let first_diff = expected
                .iter()
                .zip(&actual)
                .position(|(left, right)| left != right)
                .unwrap_or(expected.len().min(actual.len()));
            mismatches.push(format!(
                "{path}: expected {} bytes, actual {} bytes; first difference at byte {first_diff}",
                expected.len(),
                actual.len()
            ));
        }
    }
    assert!(
        mismatches.is_empty(),
        "generated contract bytes changed under core.autocrlf=true (unprotected CRLF control passed):\n{}",
        mismatches.join("\n")
    );
}
