//! Retained-state initialization and initial SDK checkout of the advertised HEAD.

use gix::{
    Progress,
    bstr::ByteSlice,
    protocol::handshake::Ref,
    refs::{
        FullName, Target,
        transaction::{PreviousValue, RefEdit},
    },
};

use super::*;

impl ProjectRepo {
    pub(super) fn clone_with_connection<P, F>(
        destination: &Path,
        address: &Url,
        mut progress: P,
        interrupt: &AtomicBool,
        connect: F,
    ) -> Result<Self, GitError>
    where
        P: gix::NestedProgress,
        P::SubProgress: 'static,
        F: for<'remote, 'repo> FnOnce(
            &'remote gix::Remote<'repo>,
        )
            -> Result<gix::remote::Connection<'remote, 'static, 'repo, Box<dyn Transport + Send>>, GitError>,
    {
        interrupted(interrupt)?;
        // create_dir, never create_dir_all: an existing empty directory or a
        // dangling symlink belongs to the caller, not to this clone.
        std::fs::create_dir(destination).map_err(|_| {
            failure("clone requires an absent destination and an existing writable parent; open an occupied repository instead")
        })?;
        let mut repo = gix::ThreadSafeRepository::init_opts(
            destination,
            gix::create::Kind::WithWorktree,
            gix::create::Options {
                destination_must_be_empty: Some(true),
                ..Default::default()
            },
            open_options(),
        )
        .map_err(|_| {
            failure("clone destination was claimed but initialization failed; partial state is retained; inspect it before recovery")
        })?
        .to_thread_local();
        // The lower-level initializer has no clone cleanup destructor. From this
        // point every error retains metadata and any downloaded/user-written data.
        repo.refs.write_reflog = gix::refs::store::WriteReflog::Disable;
        let completed = (|| {
            repository(&repo)?;
            interrupted(interrupt)?;
            let mut remote = tracking_remote(&repo, address)?;
            persist_origin(&repo, &mut remote)?;
            let outcome = fetch::receive(&repo, &remote, &mut progress, interrupt, connect)?;
            let (branch, target) = advertised_head(&outcome.ref_map)?;
            let mut index = checkout_index(&repo, target)?;
            interrupted(interrupt)?;
            // The explicit tree/index drives checkout; a born HEAD is not required.
            // Keep HEAD unborn on collision or index failure so explicit recovery
            // never has to bypass existing checkout's missing-index protection.
            checkout(&repo, &mut index, &mut progress, interrupt)?;
            publish_head(&repo, branch, target)
        })();
        completed.map_err(retained)?;
        Ok(Self { repo })
    }
}

fn retained(error: GitError) -> GitError {
    // All called transport-stage errors are bounded here, never SDK Display.
    let GitError::Git(message) = error else {
        return failure(
            "clone initialized state retained; reopen, correct the cause, fetch, then explicitly create/select and check out a branch",
        );
    };
    GitError::Git(format!(
        "clone initialized repository retained; download, refs or checkout may be partial: {message}; \
         reopen with ProjectRepo::open, correct the cause, explicitly fetch, then create/select and check out the desired branch; \
         do not retry clone over this destination"
    ))
}

fn persist_origin(repo: &gix::Repository, remote: &mut gix::Remote<'_>) -> Result<(), GitError> {
    // Write the physical local file, not the resolved config with transient
    // HTTP/trace/identity overrides. FileTransaction owns the SDK config lock.
    let mut config = repo
        .config_file_mut(repo.common_dir().join("config"))
        .map_err(|_| failure("cannot lock initialized clone configuration; correct the obstruction and recover explicitly"))?;
    remote
        .save_as_to("origin", &mut config)
        .map_err(|_| failure("cannot prepare the credential-free clone origin configuration"))?;
    config
        .commit()
        .map_err(|_| failure("cannot publish clone origin configuration; inspect initialized state"))
}

fn advertised_head(map: &gix::remote::fetch::RefMap) -> Result<(FullName, gix::ObjectId), GitError> {
    let head = map.remote_refs.iter().find(|reference| reference.unpack().0 == "HEAD");
    let Some(Ref::Symbolic {
        target, object, tag: None, ..
    }) = head
    else {
        return Err(if map.mappings.is_empty() {
            failure("remote is empty or unborn; no commit or default branch was fabricated")
        } else {
            failure(
                "remote HEAD/default branch is missing, unborn or not a symbolic branch; explicitly select a fetched branch for recovery",
            )
        });
    };
    let target_text = std::str::from_utf8(target)
        .map_err(|_| failure("remote HEAD/default branch cannot be represented without conversion; explicitly select a fetched branch"))?;
    let branch = crate::commit::validate_branch_reference(target_text)
        .map_err(|_| failure("remote HEAD must select a valid full local branch; no unrelated branch was selected"))?;
    if !map
        .mappings
        .iter()
        .any(|mapping| mapping.remote.as_name() == Some(target.as_bstr()) && mapping.remote.as_id() == Some(object.as_ref()))
    {
        return Err(failure(
            "remote HEAD/default branch was not fetched; explicitly select an available tracking branch",
        ));
    }
    Ok((branch, *object))
}

fn checkout_index(repo: &gix::Repository, target: gix::ObjectId) -> Result<gix::index::File, GitError> {
    let commit = repo
        .find_commit(target)
        .map_err(|_| failure("downloaded default-branch commit is missing or corrupt; correct the remote"))?;
    let tree = commit
        .decode()
        .map_err(|_| failure("downloaded default-branch commit is corrupt; correct the remote"))?
        .tree();
    let index = repo
        .index_from_tree(&tree)
        .map_err(|_| failure("downloaded checkout tree is missing, corrupt or contains unsafe paths"))?;
    for entry in index.entries() {
        if entry.mode.is_submodule() {
            return Err(failure(
                "clone checkout does not support gitlinks/submodules; no gitlink was written",
            ));
        }
        repo.find_object(entry.id)
            .and_then(|object| object.try_into_blob())
            .map_err(|_| failure("downloaded checkout blob is missing or corrupt; correct the remote before checkout"))?;
    }
    Ok(index)
}

fn publish_head(repo: &gix::Repository, branch: FullName, target: gix::ObjectId) -> Result<(), GitError> {
    let previous = repo
        .head()
        .map_err(|_| failure("cannot inspect initialized clone HEAD before publication"))?;
    if !previous.is_unborn()
        || repo
            .try_find_reference(&branch)
            .map_err(|_| failure("cannot inspect initial clone branch"))?
            .is_some()
    {
        return Err(failure(
            "initialized clone HEAD or local branch changed; inspect retained state instead of overwriting it",
        ));
    }
    let previous_name = previous
        .referent_name()
        .ok_or_else(|| failure("initialized clone HEAD is no longer symbolic"))?
        .to_owned();
    let head = FullName::try_from("HEAD").map_err(|_| failure("cannot prepare initial clone HEAD"))?;
    let edits = [
        RefEdit::update(branch.clone(), target, PreviousValue::MustNotExist, "clone: advertised branch").with_deref(false),
        RefEdit::update(
            head,
            Target::Symbolic(branch),
            PreviousValue::MustExistAndMatch(Target::Symbolic(previous_name)),
            "clone: advertised HEAD",
        )
        .with_deref(false),
    ];
    let edits = repo
        .refs
        .transaction()
        .prepare(edits, gix::lock::acquire::Fail::Immediately, gix::lock::acquire::Fail::Immediately)
        .and_then(|transaction| transaction.commit(None))
        .map_err(|_| failure("cannot publish initial clone branch/HEAD; inspect competing locks and retained partial refs"))?;
    if edits.first().is_none_or(|edit| edit.change.previous_value().is_some()) {
        return Err(failure(
            "initial clone branch changed concurrently; inspect retained refs before selecting a branch",
        ));
    }
    Ok(())
}

fn checkout<P>(repo: &gix::Repository, index: &mut gix::index::File, mut progress: P, interrupt: &AtomicBool) -> Result<(), GitError>
where
    P: gix::NestedProgress,
    P::SubProgress: 'static,
{
    interrupted(interrupt)?;
    let workdir = repo
        .workdir()
        .ok_or_else(|| failure("initialized clone has no working directory"))?;
    let mut options = repo
        .checkout_options(gix::worktree::stack::state::attributes::Source::IdMapping)
        .map_err(|_| failure("cannot prepare initial checkout options; correct repository configuration"))?;
    options.destination_is_initially_empty = true;
    options.overwrite_existing = false;
    options.keep_going = false;
    let mut files = progress.add_child("Git checkout files");
    let mut bytes = progress.add_child("Git checkout bytes");
    files.init(Some(index.entries().len()), gix::progress::count("files"));
    bytes.init(None, gix::progress::bytes());
    progress.info("Initial Git checkout started".to_string());
    // This SDK entry point accepts only Count, not Progress/NestedProgress:
    // live numeric counters cross the boundary, but no backend names/messages do.
    let outcome = gix::worktree::state::checkout(
        index,
        workdir,
        repo.objects
            .clone()
            .into_arc()
            .map_err(|_| failure("cannot open downloaded objects for initial checkout"))?,
        &files,
        &bytes,
        interrupt,
        options,
    );
    interrupted(interrupt)?;
    let outcome = outcome.map_err(|_| failure("initial checkout failed; downloaded objects, refs and partial worktree are retained"))?;
    if !outcome.collisions.is_empty()
        || !outcome.errors.is_empty()
        || !outcome.delayed_paths_unknown.is_empty()
        || !outcome.delayed_paths_unprocessed.is_empty()
        || outcome.files_updated != index.entries().len()
    {
        return Err(failure(
            "initial checkout is incomplete or collided with existing paths; preserve content and explicitly recover",
        ));
    }
    index
        .write(Default::default())
        .map_err(|_| failure("checkout wrote worktree files but could not publish the index; recover explicitly"))?;
    interrupted(interrupt)?;
    progress.info("Initial Git checkout completed".to_string());
    Ok(())
}
