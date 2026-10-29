//! Reference/object/actor failures are actionable and prewrite.

use super::*;
use crate::commit::validation::absent_object;

isolated_tests! {
fn invalid_reference(reference: &str) {
    let dir = fixture();
    let repo = ProjectRepo::open(dir.path()).unwrap();

    deny(
        &repo,
        dir.path(),
        reference,
        actor(),
        &["reference", "branch", "refs/heads", "name"],
    );

    switch(&repo, dir.path());
}

#[test]
fn checkout_head_alias_denies_then_full_reference_succeeds() {
    invalid_reference("HEAD");
}

#[test]
fn checkout_short_name_denies_then_full_reference_succeeds() {
    invalid_reference("topic");
}

#[test]
fn checkout_nonlocal_reference_denies_then_local_reference_succeeds() {
    for reference in ["refs/tags/topic", "refs/remotes/origin/topic"] {
        invalid_reference(reference);
    }
}

#[test]
fn checkout_malformed_reference_denies_then_valid_reference_succeeds() {
    for reference in [
        "refs/heads/",
        "refs/heads/../escape",
        "refs/heads/topic.lock",
        "refs/heads/bad\0name",
        "refs/heads/bad\nname",
    ] {
        invalid_reference(reference);
    }
}

#[test]
fn checkout_missing_branch_denies_then_existing_branch_succeeds() {
    let dir = fixture();
    let repo = ProjectRepo::open(dir.path()).unwrap();

    deny(
        &repo,
        dir.path(),
        "refs/heads/missing",
        actor(),
        &["missing", "not found", "absent", "exist"],
    );

    switch(&repo, dir.path());
}

#[test]
fn checkout_symbolic_target_denies_without_dereferencing_then_direct_target_succeeds() {
    let dir = fixture();
    let target = resolve(dir.path(), REFERENCE);
    fs::write(dir.path().join(".git").join(REFERENCE), b"ref: refs/heads/main\n").unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();

    deny_default(&repo, dir.path(), &["symbolic", "direct"]);

    replace_target(dir.path(), target);
    switch(&repo, dir.path());
}

#[test]
fn checkout_malformed_target_ref_denies_then_repaired_reference_succeeds() {
    let dir = fixture();
    let target = resolve(dir.path(), REFERENCE);
    fs::write(dir.path().join(".git").join(REFERENCE), b"not an object id\n").unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();

    deny_default(&repo, dir.path(), &["reference", "branch", "target"]);

    replace_target(dir.path(), target);
    switch(&repo, dir.path());
}

fn invalid_target(dir: &Path, invalid: gix::ObjectId) {
    let target = resolve(dir, REFERENCE);
    replace_target(dir, invalid);
    let repo = ProjectRepo::open(dir).unwrap();

    deny_default(&repo, dir, &["commit", "object"]);

    replace_target(dir, target);
    switch(&repo, dir);
}

#[test]
fn checkout_missing_target_object_denies_then_available_commit_succeeds() {
    let dir = fixture();
    invalid_target(dir.path(), absent_object(dir.path()));
}

#[test]
fn checkout_blob_target_denies_then_commit_target_succeeds() {
    let dir = fixture();
    invalid_target(dir.path(), resolve(dir.path(), "HEAD:tracked.txt"));
}

#[test]
fn checkout_tree_target_denies_then_commit_target_succeeds() {
    let dir = fixture();
    invalid_target(dir.path(), resolve(dir.path(), "HEAD^{tree}"));
}

#[test]
fn checkout_annotated_tag_target_denies_without_peeling_then_commit_succeeds() {
    let dir = fixture();
    git(dir.path(), &["tag", "-a", "fixture", "-m", "Annotated fixture", REFERENCE]);
    invalid_target(dir.path(), resolve(dir.path(), "refs/tags/fixture"));
}

#[test]
fn checkout_malformed_commit_target_denies_then_decodable_commit_succeeds() {
    let dir = fixture();
    invalid_target(dir.path(), malformed_commit(dir.path()));
}

fn damaged_object(revision: &str, corrupt: bool, categories: &[&str]) {
    let dir = fixture();
    let object = loose_object(dir.path(), resolve(dir.path(), revision));
    let saved = fs::read(&object).unwrap();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    fs::remove_file(&object).unwrap();
    if corrupt {
        fs::write(&object, b"checkout-raw-canary corrupt loose object\n").unwrap();
    }

    deny_default(&repo, dir.path(), categories);

    fs::write(object, saved).unwrap();
    switch(&repo, dir.path());
}

#[test]
fn checkout_corrupt_target_commit_denies_then_repaired_object_succeeds() {
    damaged_object(REFERENCE, true, &["commit", "object"]);
}

#[test]
fn checkout_missing_target_tree_denies_before_any_write_then_repair_succeeds() {
    damaged_object("topic^{tree}", false, &["tree", "object"]);
}

#[test]
fn checkout_corrupt_target_tree_denies_before_any_write_then_repair_succeeds() {
    damaged_object("topic^{tree}", true, &["tree", "object"]);
}

#[test]
fn checkout_missing_target_blob_denies_before_any_write_then_repair_succeeds() {
    damaged_object("topic:z-new", false, &["blob", "object"]);
}

#[test]
fn checkout_corrupt_target_blob_denies_before_any_write_then_repair_succeeds() {
    damaged_object("topic:z-new", true, &["blob", "object"]);
}

#[test]
fn checkout_missing_baseline_commit_denies_before_any_write_then_repair_succeeds() {
    damaged_object("main", false, &["commit", "object", "head", "baseline"]);
}

#[test]
fn checkout_corrupt_baseline_tree_denies_before_any_write_then_repair_succeeds() {
    damaged_object("main^{tree}", true, &["tree", "object", "baseline"]);
}

#[test]
fn checkout_missing_baseline_blob_denies_before_any_write_then_repair_succeeds() {
    damaged_object("main:tracked.txt", false, &["blob", "object", "baseline"]);
}

fn invalid_actor(committer: gix::actor::SignatureRef<'_>, categories: &[&str]) {
    let dir = fixture();
    let repo = ProjectRepo::open(dir.path()).unwrap();

    deny(&repo, dir.path(), REFERENCE, committer, categories);

    switch(&repo, dir.path());
}

#[test]
fn checkout_invalid_actor_name_denies_without_disclosure_then_valid_actor_succeeds() {
    for field in [
        b"".as_slice(),
        b" ",
        b"bad\0name",
        b"bad\rname",
        b"bad<name",
        b"bad>name",
        b"checkout-raw-canary\n",
    ] {
        invalid_actor(
            gix::actor::SignatureRef {
                name: field.as_bstr(),
                ..actor()
            },
            &["name"],
        );
    }
}

#[test]
fn checkout_invalid_actor_email_denies_without_disclosure_then_valid_actor_succeeds() {
    for field in [
        b"".as_slice(),
        b" ",
        b"bad\0email",
        b"bad\remail",
        b"bad<email",
        b"bad>email",
        b"checkout-raw-canary\n",
    ] {
        invalid_actor(
            gix::actor::SignatureRef {
                email: field.as_bstr(),
                ..actor()
            },
            &["email"],
        );
    }
}

#[test]
fn checkout_invalid_actor_time_denies_without_disclosure_then_valid_actor_succeeds() {
    for time in [
        "checkout-raw-canary",
        "",
        "946688400 +9999",
        "9223372036854775808 +0000",
        "946688400 +0530\ncheckout-raw-canary",
    ] {
        invalid_actor(gix::actor::SignatureRef { time, ..actor() }, &["time"]);
    }
}

#[test]
fn checkout_normalized_actor_time_denies_instead_of_losing_metadata() {
    for time in ["0946688400 +0530", "946688400 -0000", "946688400 +05:30"] {
        invalid_actor(gix::actor::SignatureRef { time, ..actor() }, &["time", "round-trip", "normaliz"]);
    }
}

#[test]
fn checkout_unrepresentable_actor_name_denies_instead_of_lossy_utf8_conversion() {
    invalid_actor(
        gix::actor::SignatureRef {
            name: b"checkout-raw-canary-\xff".as_bstr(),
            ..actor()
        },
        &["name", "utf-8", "represent"],
    );
}

#[test]
fn checkout_unrepresentable_actor_email_denies_instead_of_lossy_utf8_conversion() {
    invalid_actor(
        gix::actor::SignatureRef {
            email: b"checkout-raw-canary-\xff@example.invalid".as_bstr(),
            ..actor()
        },
        &["email", "utf-8", "represent"],
    );
}

#[test]
fn checkout_normalized_actor_name_denies_instead_of_trimming_metadata() {
    for name in [b" Checkout Actor".as_slice(), b"Checkout Actor "] {
        invalid_actor(
            gix::actor::SignatureRef {
                name: name.as_bstr(),
                ..actor()
            },
            &["name", "represent", "normaliz"],
        );
    }
}

#[test]
fn checkout_normalized_actor_email_denies_instead_of_trimming_metadata() {
    for email in [b" checkout@example.invalid".as_slice(), b"checkout@example.invalid "] {
        invalid_actor(
            gix::actor::SignatureRef {
                email: email.as_bstr(),
                ..actor()
            },
            &["email", "represent", "normaliz"],
        );
    }
}
}
