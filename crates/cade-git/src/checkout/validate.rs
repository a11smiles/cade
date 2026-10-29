//! Fresh prewrite validation and lossless conversion at the checkout SDK boundary.

use std::{collections::HashSet, fs, path::Path};

use gix::objs::tree::EntryKind;

use crate::{GitError, commit::validate_signature, ge, has_repository_metadata, path_entry_exists, reject_executable_filters};

pub(super) fn actor(input: gix::actor::SignatureRef<'_>) -> Result<git2::Signature<'static>, GitError> {
    let actor = input
        .to_owned()
        .map_err(|_| GitError::Git("cannot parse checkout actor timestamp".into()))?;
    // Keep the shared validator unchanged, but label its SDK time-serialization
    // failure here without exposing the supplied timestamp or SDK error payload.
    actor
        .time
        .write_to(&mut std::io::sink())
        .map_err(|_| GitError::Git("checkout actor time cannot be represented".into()))?;
    validate_signature(&actor)?;
    let mut time = gix::date::parse::TimeBuf::default();
    if actor.to_ref(&mut time).time != input.time {
        return Err(GitError::Git("checkout actor time must round-trip without normalization".into()));
    }
    let name = std::str::from_utf8(&actor.name)
        .map_err(|_| GitError::Git("checkout actor name must be representable as UTF-8 without conversion".into()))?;
    let email = std::str::from_utf8(&actor.email)
        .map_err(|_| GitError::Git("checkout actor email must be representable as UTF-8 without conversion".into()))?;
    let time = git2::Time::new(actor.time.seconds, actor.time.offset / 60);
    let converted = git2::Signature::new(name, email, &time)
        .map_err(|_| GitError::Git("checkout actor name/email/time cannot be represented by the checkout SDK".into()))?;
    if converted.name_bytes() != input.name {
        return Err(GitError::Git("checkout actor name would be normalized".into()));
    }
    if converted.email_bytes() != input.email {
        return Err(GitError::Git("checkout actor email would be normalized".into()));
    }
    if converted.when() != time {
        return Err(GitError::Git("checkout actor time would be normalized".into()));
    }
    Ok(converted)
}

pub(super) fn reopen(path: &Path) -> Result<gix::Repository, GitError> {
    gix::open_opts(path, gix::open::Options::default().strict_config(true).open_path_as_is(true))
        .map_err(|_| GitError::Git("cannot reopen checkout repository metadata/configuration; repair it and explicitly retry".into()))
}

pub(super) fn repository(repo: &gix::Repository) -> Result<(), GitError> {
    let workdir = repo
        .workdir()
        .ok_or_else(|| GitError::Git("checkout requires a working tree; bare repositories are unsupported".into()))?;
    repo.head()
        .map_err(|_| GitError::Git("cannot read checkout HEAD reference".into()))?;
    reject_executable_filters(repo)?;
    let config = repo.config_snapshot();
    for section in config.plumbing().sections_by_name("filter").into_iter().flatten() {
        if section.body().value_implicit("smudge").is_some() {
            return Err(GitError::Git(
                "checkout does not support executable smudge filters; remove the filter configuration and explicitly retry".into(),
            ));
        }
    }
    for marker in [
        "MERGE_HEAD",
        "CHERRY_PICK_HEAD",
        "REVERT_HEAD",
        "rebase-merge",
        "rebase-apply",
        "sequencer",
    ] {
        if path_entry_exists(&repo.git_dir().join(marker))? {
            return Err(GitError::Git(format!(
                "checkout refuses an unresolved operation ({marker}); finish or explicitly correct that operation before retrying"
            )));
        }
    }
    regular_file(&repo.git_dir().join("logs/HEAD"), "HEAD reflog")?;
    reject_nested_repositories(workdir)
}

// Inspect the configuration the fallback itself will use as well. This does
// not install callbacks, invoke executables or silently disable configured rules.
pub(super) fn sdk_filters(repo: &git2::Repository) -> Result<(), GitError> {
    let config = repo
        .config()
        .map_err(|_| GitError::Git("cannot read checkout SDK configuration".into()))?;
    let mut entries = config
        .entries(None)
        .map_err(|_| GitError::Git("cannot inspect checkout SDK filter configuration".into()))?;
    while let Some(entry) = entries.next() {
        let entry = entry.map_err(|_| GitError::Git("cannot decode checkout SDK filter configuration".into()))?;
        let key = entry.name_bytes();
        if key.starts_with(b"filter.")
            && [b".clean".as_slice(), b".smudge", b".process"]
                .iter()
                .any(|suffix| key.ends_with(suffix))
        {
            return Err(GitError::Git(
                "checkout does not support executable clean/smudge/process filters; remove the filter configuration and explicitly retry"
                    .into(),
            ));
        }
    }
    Ok(())
}

pub(super) fn regular_file(path: &Path, label: &str) -> Result<(), GitError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.is_file() && !metadata.permissions().readonly() => Ok(()),
        Ok(_) => Err(GitError::Git(format!(
            "checkout {label} is obstructed or not a writable regular file"
        ))),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            // A missing log/index is supported, but a known obstructed parent
            // must not be discovered only after the SDK changes user files.
            if let Some(parent) = path.parent() {
                directory_parent(parent, label)?;
            }
            Ok(())
        }
        Err(error) => Err(GitError::Git(format!("cannot inspect checkout {label}: {error}"))),
    }
}

fn directory_parent(path: &Path, label: &str) -> Result<(), GitError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.is_dir() && !metadata.permissions().readonly() => Ok(()),
        Ok(_) => Err(GitError::Git(format!("checkout {label} parent is obstructed or not writable"))),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => match path.parent() {
            Some(parent) => directory_parent(parent, label),
            None => Err(GitError::Git(format!("checkout {label} has no existing parent"))),
        },
        Err(error) => Err(GitError::Git(format!("cannot inspect checkout {label} parent: {error}"))),
    }
}

fn reject_nested_repositories(root: &Path) -> Result<(), GitError> {
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(&directory).map_err(ge)? {
            let entry = entry.map_err(ge)?;
            if directory == root && entry.file_name() == ".git" {
                continue;
            }
            // Never descend through a symlink, including native directory links.
            if !entry.file_type().map_err(ge)?.is_dir() {
                continue;
            }
            let path = entry.path();
            if has_repository_metadata(&path)? {
                return Err(GitError::Git(
                    "checkout does not support nested repository traversal; inspect the nested repository separately".into(),
                ));
            }
            pending.push(path);
        }
    }
    Ok(())
}

pub(super) fn commit_tree(repo: &gix::Repository, id: gix::ObjectId) -> Result<(), GitError> {
    let commit = repo
        .find_commit(id)
        .map_err(|_| GitError::Git(format!("cannot read checkout commit object {id}")))?;
    let tree = commit
        .decode()
        .map_err(|_| GitError::Git(format!("cannot decode checkout commit object {id}")))?
        .tree();
    let mut pending = vec![tree];
    let mut seen = HashSet::new();
    while let Some(id) = pending.pop() {
        if !seen.insert(id) {
            continue;
        }
        let tree = repo
            .find_tree(id)
            .map_err(|_| GitError::Git(format!("cannot read checkout tree object {id}")))?;
        let decoded = tree
            .decode()
            .map_err(|_| GitError::Git(format!("cannot decode checkout tree object {id}")))?;
        for entry in decoded.entries {
            match entry.mode.kind() {
                EntryKind::Tree => pending.push(entry.oid.to_owned()),
                EntryKind::Commit => {
                    return Err(GitError::Git("checkout does not support gitlinks/submodules".into()));
                }
                EntryKind::Blob | EntryKind::BlobExecutable | EntryKind::Link => {
                    repo.find_object(entry.oid)
                        .and_then(|object| object.try_into_blob())
                        .map_err(|_| GitError::Git(format!("cannot read checkout blob object {}", entry.oid)))?;
                }
            }
        }
    }
    Ok(())
}
