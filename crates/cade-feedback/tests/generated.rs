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
fn forms_checked_in_artifact_matches_hosted_yaml() {
    assert_eq!(
        tracked("app/ui/src/feedback/forms.generated.ts"),
        support::forms().expect("hosted YAML generates form metadata"),
        "run cargo run -p cade-feedback --example generate_feedback -- forms and apply stdout"
    );
}
