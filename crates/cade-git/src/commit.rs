//! Explicit immutable commit selection and guarded local branch publication.

use gix::bstr::ByteSlice;
use gix::objs::WriteTo;
use gix::refs::{
    FullName, Target,
    transaction::{PreviousValue, RefEdit},
};

use crate::{GitError, ProjectRepo, ge};

impl ProjectRepo {
    /// Commit an explicitly selected SDK tree to a fully qualified local branch.
    ///
    /// The first parent must match the existing direct branch tip; no parents
    /// requires an absent branch. Supplied metadata and message bytes, including
    /// an empty message, are preserved. Nothing is staged; HEAD attachment is unchanged.
    /// Hooks, filters, signing programs and network operations are not invoked.
    ///
    /// Locks and stale tips fail without retry. A failed reference transaction
    /// may leave an unreachable commit object; existing objects are never removed.
    pub fn commit(&self, reference: &str, commit: &gix::objs::Commit) -> Result<gix::ObjectId, GitError> {
        let reference = validate_branch_reference(reference)?;
        // Fresh handles allow explicitly corrected requests to recover without
        // reusing stale configuration, packed references or object caches.
        let repo = gix::open_opts(
            self.repo.git_dir(),
            gix::open::Options::default().strict_config(true).open_path_as_is(true),
        )
        .map_err(|error| GitError::Git(format!("cannot reopen repository for commit: {error:#}")))?;
        validate_commit_headers(commit, repo.object_hash())?;
        validate_commit_objects(&repo, commit)?;

        // Reject observed mismatches before locks or writes so SDK ref-lock
        // rollback cannot prune a preexisting empty directory on these denials.
        // Publication still checks the strict expected tip under lock for races.
        validate_commit_reference(&repo, &reference, commit)?;

        // Hold only: dropping this marker removes our lock, never the index.
        let _index_lock = lock_index(&repo)?;
        let id = repo
            .write_object(commit)
            .map_err(|error| GitError::Git(format!("cannot write commit object: {error:#}")))?
            .detach();
        publish_commit(&repo, reference, commit, id)?;
        Ok(id)
    }
}

pub(super) fn validate_branch_reference(reference: &str) -> Result<FullName, GitError> {
    if !reference.starts_with("refs/heads/") {
        return Err(GitError::Git(
            "commit requires a fully qualified local branch under refs/heads/".into(),
        ));
    }
    gix::validate::reference::branch_name(reference.as_bytes().as_bstr()).map_err(ge)?;
    FullName::try_from(reference).map_err(ge)
}

fn validate_commit_reference(repo: &gix::Repository, reference: &FullName, commit: &gix::objs::Commit) -> Result<(), GitError> {
    let observed = repo
        .try_find_reference(reference)
        .map_err(|error| GitError::Git(format!("cannot inspect commit reference {reference}: {error:#}")))?;
    match (commit.parents.first(), observed) {
        (Some(_), None) => Err(GitError::Git(format!(
            "parented commit requires an existing branch; {reference} is absent"
        ))),
        (Some(parent), Some(observed)) if observed.try_id().map(|id| id.detach()) != Some(*parent) => Err(GitError::Git(format!(
            "cannot lock or prepare commit reference {reference}: parented commit requires a direct branch matching first parent {parent}"
        ))),
        (None, Some(_)) => Err(GitError::Git(format!(
            "initial commit requires an absent branch; {reference} already exists"
        ))),
        _ => Ok(()),
    }
}

fn validate_commit_objects(repo: &gix::Repository, commit: &gix::objs::Commit) -> Result<(), GitError> {
    repo.find_object(commit.tree)
        .map_err(|error| GitError::Git(format!("cannot read selected tree {}: {error:#}", commit.tree)))?
        .try_into_tree()
        .map_err(ge)?
        .decode()
        .map_err(ge)?;
    for parent in &commit.parents {
        repo.find_object(*parent)
            .map_err(|error| GitError::Git(format!("cannot read parent commit {parent}: {error:#}")))?
            .try_into_commit()
            .map_err(ge)?
            .decode()
            .map_err(ge)?;
    }
    Ok(())
}

pub(super) fn validate_signature(signature: &gix::actor::Signature) -> Result<(), GitError> {
    for (name, field) in [("name", &signature.name), ("email", &signature.email)] {
        // The SDK retains invalid serialization tokens in its error metadata.
        // Reject them before serialization without including supplied identity bytes.
        if field.trim().is_empty() || field.iter().any(|byte| matches!(byte, b'\0' | b'\r' | b'\n' | b'<' | b'>')) {
            return Err(GitError::Git(format!(
                "commit identity {name} must be nonempty and contain no NUL, carriage return, newline or angle brackets"
            )));
        }
    }
    // Time::to_str, used by SDK signature serialization, assumes a representable
    // offset. Check it fallibly first, and reject subminute loss of metadata.
    signature.time.write_to(&mut std::io::sink()).map_err(ge)?;
    if signature.time.offset % 60 != 0 {
        return Err(GitError::Git("commit identity timezone offsets must use whole minutes".into()));
    }
    Ok(())
}

fn validate_commit_headers(commit: &gix::objs::Commit, hash: gix::hash::Kind) -> Result<(), GitError> {
    validate_signature(&commit.author)?;
    validate_signature(&commit.committer)?;
    if commit
        .encoding
        .as_ref()
        .is_some_and(|encoding| encoding.iter().any(|byte| matches!(byte, b'\0' | b'\r')))
    {
        return Err(GitError::Git(
            "commit encoding must be a single header value without NUL or carriage returns".into(),
        ));
    }
    for (name, value) in &commit.extra_headers {
        if name.is_empty() || name.iter().any(|byte| byte.is_ascii_whitespace() || byte.is_ascii_control()) {
            return Err(GitError::Git(
                "extra commit header names must be nonempty tokens without whitespace or controls".into(),
            ));
        }
        if matches!(&name[..], b"tree" | b"parent" | b"author" | b"committer" | b"encoding") {
            return Err(GitError::Git("extra commit headers must not duplicate structural headers".into()));
        }
        if value.iter().any(|byte| matches!(byte, b'\0' | b'\r')) {
            return Err(GitError::Git(
                "extra commit header values must not contain NUL or carriage returns".into(),
            ));
        }
    }
    // Let the SDK validate signatures, single-line encoding and folded headers.
    // No hand-written object codec, normalization or message rewriting.
    let mut serialized = Vec::new();
    commit.write_to(&mut serialized).map_err(ge)?;
    gix::objs::CommitRef::from_bytes(&serialized, hash).map_err(ge)?;
    Ok(())
}

fn lock_index(repo: &gix::Repository) -> Result<gix::lock::Marker, GitError> {
    let index = repo.index_path();
    gix::lock::Marker::acquire_to_hold_resource(&index, gix::lock::acquire::Fail::Immediately, None, 0)
        .map_err(|error| GitError::Git(format!("cannot lock index {}: {error:#}", index.display())))
}

fn publish_commit(repo: &gix::Repository, reference: FullName, commit: &gix::objs::Commit, id: gix::ObjectId) -> Result<(), GitError> {
    let expected = match commit.parents.first() {
        Some(parent) => PreviousValue::MustExistAndMatch(Target::Object(*parent)),
        None => PreviousValue::MustNotExist,
    };
    let edit = RefEdit::update(
        reference.clone(),
        id,
        expected,
        gix::reference::log::message("commit", commit.message.as_bstr(), commit.parents.len()),
    )
    .with_deref(false);
    let transaction = repo
        .refs
        .transaction()
        .prepare([edit], gix::lock::acquire::Fail::Immediately, gix::lock::acquire::Fail::Immediately)
        .map_err(|error| GitError::Git(format!("cannot lock or prepare commit reference {reference}: {error:#}")))?;
    let mut time = gix::date::parse::TimeBuf::default();
    let edits = transaction
        .commit(Some(commit.committer.to_ref(&mut time)))
        .map_err(|error| GitError::Git(format!("cannot update commit reference {reference}: {error:#}")))?;
    // The SDK treats MustNotExist with an identical existing target as a no-op:
    // neither ref nor reflog changes, and its returned edit records the old tip.
    // Reject that outcome too, so a repeated initial request cannot claim success.
    if commit.parents.is_empty() && edits.iter().any(|edit| edit.change.previous_value().is_some()) {
        return Err(GitError::Git(format!(
            "initial commit requires an absent branch; {reference} already exists"
        )));
    }
    Ok(())
}
