# CADE architecture

## Intended architecture

The diagram and principles below describe the intended architecture, not a
completed integration inventory. An available crate does not establish that the
desktop app invokes all of its capabilities.

```
┌──────────────────────── CADE window (Tauri 2) ────────────────────────┐
│  app/ui  React + TypeScript webview (transparent)                      │
│  top bar · panels · Copilot column · properties strip · dialogs        │
│ ─────────────────────────── Tauri IPC ──────────────────────────────── │
│  app/src-tauri (cade-app)  commands, state, windowing                  │
│     │                                                                   │
│     ├─ cade-render  wgpu surface under the webview, Vello scenes        │
│     │     ├─ cade-text   Parley                                         │
│     │     └─ cade-layout Taffy                                          │
│     ├─ cade-crdt    Loro document (.cade) ── cade-core types            │
│     ├─ cade-git     gix; checkout/push-upload git2/libgit2 fallback    │
│     │     └─ cade-github → cade-policy  Git network boundary           │
│     ├─ cade-copilot GitHub Copilot SDK (Copilot or BYOK providers)      │
│     │     └─ cade-policy  every tool call checked against policy.yml    │
│     ├─ cade-github  device flow, PRs, Pages review, issues (feedback)   │
│     ├─ cade-feedback bug / feature / comment → redacted issue           │
│     ├─ importers    cade-io-visio · -pptx · -figma · -spark             │
│     ├─ generators   cade-codegen-react · cade-iac · cade-ddl            │
│     ├─ cade-system  Data → App → Architecture bindings and deploy plan  │
│     └─ cade-io-export  PNG SVG PDF PPTX DOCX MP4 GIF Lottie VSDX …      │
└────────────────────────────────────────────────────────────────────────┘
```

- **Local first.** Projects are folders and git repos on the user's machine. Nothing is uploaded except model calls (to Copilot or the user's own provider) and what the user ships (PRs, review pages, issues).
- **One identity.** GitHub device-flow sign-in is the intended identity source for Copilot, shipping, review pages and feedback issues. Git clone/fetch/push require an explicitly supplied token already associated with the selected Host; the existing global keychain entry alone does not establish that association.
- **Policy everywhere.** `cade-policy` decides before Copilot runs a tool, before BYOK is used, and before anything ships.
- **Copilot proposes, the user applies.** Agent output is a staged change set. Applying it is one commit with run trailers.

See `docs/adr/` for the decisions behind this; the accepted
[ADRs](adr/) remain unchanged.

## Delivered capabilities and boundaries

The [README crate table](../README.md#implemented-starting-points) inventories
existing code. The following sections distinguish implemented remediation from
bounded approval and pending verification; none establishes release readiness.

### Git implementation boundary

**Implementation inventory:** local open/init/status foundation (#596 / GH-024), explicit-tree
commits (#597 / GH-025), branch creation/merge-base (#598 / GH-026 Stage A)
and existing-worktree checkout (Stage B) are implemented.
The following acceptance chronology is historical, reconciled at the
October 27, 2026, 22:38:54 −07:00 documentation input checkpoint.
Stage A and checkout
at repair commit `ed093023794b791057bbe308d2c86fd0f173af66`, tree
`e9bb58276ce84dce751cad214a3e7b9f6a8eddbf`, have independent local acceptance
and all required native CI successes. PR #614 is READY with actual head/base/
body readback complete; CodeQL was NOT_TRIGGERED, not PASS.
Explicit clone/fetch (#599 / GH-027)
is implemented in the 21 product paths committed at
`ed53dba4f90f6e6cd24355738ff19047fde99bd5`. Independent original-scenario
recheck passed 10/10 probes and 6/6 configuration regressions; source verification
and the bounded macOS implementation/docs QA recommendation passed.
Guarded workspace Clippy, app build and 12 app tests passed at that source.
Earlier loopback/cache-copy denials are resolved by existing explicit approval;
the #599 pause is released following #598's required native CI success.
The later #599 parent `0fb8fcc4968f8553bcc6fcf29a9ab8f834745f16`,
tree `45124332f3f8a8625b48906c0a987a6a55e7f6e6`, has independently accepted
12/12 publication cases and twelve local gates. PM's October 27,
20:14:24 −07:00 receipt records PR #615 READY, with native successes at the
20:03:59 architect receipt; its earlier draft/native-wait wording is historical.
The 22:38:54 relay retains PRs #611–#615 READY and #608 fully accepted.
These are named-parent results, not #600 or full-original acceptance.
Each successor requires separately bound full local gates, independent QA and
applicable exact-head native checks; earlier-tree results do not transfer.
Four earlier unguarded fmt/UI successes remain historical, not substitutes
for the later named-parent evidence.
The bounded #600 push implementation is described below; it is no longer
an unimplemented contract waiting for #599 gates or user merge.
These bounded dispositions do not establish all original #598/#599/#600/#50
criteria or authorize issue closure under
[ADR 0004](adr/0004-git-is-history.md), the unchanged Git-history authority.

The approved local contract is implemented in
[`cade-git`](../crates/cade-git/src/lib.rs). Existing `ProjectRepo` and
`GitError` remain the API, with
`ProjectRepo::status(&self) -> Result<Vec<gix::status::Item>, GitError>`.
Consumers are crate callers/tests, not app IPC; `cade-app` does not depend on
this crate. There is no new shared status wrapper, serialized format, `.cade`
or policy schema. Open/init/status, HEAD inspection, commits, branch creation
and ancestry remain on gix, as do clone/fetch and clone's initial checkout.
Existing-worktree checkout and push upload use the feature-specific
git2/libgit2 fallback described below. Checkout retains the existing workspace
`tempfile` dependency for its private index; push validation and fresh
remote observations remain on gix.

- **Open/init:** opening preserves existing metadata and worktree contents,
  including support for opening bare repositories. Strict init rejects existing
  repositories and occupied metadata, including `.git` files, directories and
  dangling symlinks, without mutation; new initialization preserves unrelated
  files. Missing/nonrepository paths retain `GitError::NotARepo`. Corrupt or
  unreadable metadata yields actionable `GitError::Git`, not a clean result.
- **Fresh, combined status:** each request reopens metadata and reads fresh
  index data, combining HEAD→index and index→worktree differences. Separate
  `TreeIndex` and `IndexWorktree` SDK events are retained, including two events
  for one path, with unspecified order and byte-preserving relative paths.
  Clean, modified, staged, deleted, unborn and conflicted states are represented;
  conflicts are not resolved. Untracked files are individual entries even when
  configuration hides them. Rename inference is not promised.
- **Read-only failure boundaries:** status does not write index stat-cache
  updates, refs, config or worktree content. Bare status and corrupt/unreadable
  metadata, index, HEAD or tracked content fail rather than report false-clean.
  Corrected subsequent requests can recover without automatic repair/reset.
  Configured external clean/process filters are rejected before execution even
  when unused; smudge-only commands are not executed. Submodule/gitlink and
  nested-repository inspection is unsupported, with actionable denial rather
  than false-clean results or recursive traversal. These operations use gix without
  subprocess Git, shell or network operations.

HEAD inspection and Copilot commit-message formatting also remain available;
the commit operation below stores supplied messages rather than generating
attribution. Remaining work includes full original #598 acceptance and #599
hosted/authentication/platform acceptance, full original #600 push acceptance
beyond the bounded implementation below, #601 configured LFS, and #602 typed
cancellable job/IPC integration. Commit-per-save/run and full Git/app integration
remain undelivered. These local mechanics preserve the parent obligation; they
do not complete the separately scoped push, LFS or IPC layers.

#### Explicit immutable-tree commits

The public method is implemented in the private
[`commit.rs`](../crates/cade-git/src/commit.rs) module:
`ProjectRepo::commit(&self, reference: &str, commit: &gix::objs::Commit) -> Result<gix::ObjectId, GitError>`.
It reuses the SDK commit type, not a custom request wrapper.

- **Caller-selected content:** the supplied tree identifies already-stored
  immutable content; supplied parents retain their order, with the first parent
  serving as the expected branch tip. The method does not stage paths, select
  from the mutable index or infer worktree content. It validates the selected
  tree and parents for availability, object kind and decoding, and validates
  identities/header structure before publishing.
- **Strict branch publication:** callers supply a valid full `refs/heads/`
  reference, not `HEAD`, a short name or a tag. A parentless initial commit
  requires `MustNotExist`; a parented commit requires a direct ref matching its
  first parent through `MustExistAndMatch`. SDK atomic compare-and-swap
  publication uses `with_deref(false)`: changed, deleted or symbolic targets
  cannot be overwritten or silently retargeted. The weaker
  `ExistingMustMatch` condition is not used. Full observed-reference preflight
  rejects missing, mismatched or symbolic targets for parented commits and any
  existing target for initial commits, including packed refs, before locks or
  object writes. This does not replace the strict under-lock publication check.
- **Preservation and locks:** an SDK index resource lock is held without
  writing or replacing the index; initial repositories without an index remain
  supported. Worktree content, config, other refs and HEAD attachment are
  preserved. Intended effects are the target ref and normal object/reflog
  additions, using the supplied committer rather than global identity. No hooks,
  external filters, signing executables, subprocess Git or network are invoked.
- **Exact SDK bytes:** stored commit bytes match SDK serialization of the
  unchanged valid input, including metadata and message bytes; empty messages
  are allowed. Applied-run callers use the existing `run_commit_message` for
  `CADE-Copilot-Run`, `CADE-Model` and `CADE-Tokens`; ordinary commits receive
  no fabricated attribution. The SDK's multiline-header decoder retains a
  terminating newline, so decoded-field checks compare against decoding those
  expected bytes. This is a storage-test distinction, not a production encoding
  change or permission to trim metadata.
- **Failures and explicit recovery:** invalid input, permissions, lock
  obstructions and stale tips return actionable `GitError::Git`. Corrected
  requests may be retried explicitly; there is no automatic retry or force
  update. Repeating a successfully published request fails without duplicating
  history or moving the branch backward. Failed ref transactions may leave an
  unreachable immutable object; failure is reported without deleting objects
  or another writer's locks. Observed-reference preflight avoids ref-lock
  rollback pruning preexisting empty directories on those known denials,
  including stale packed refs. It does not guarantee directory preservation
  after a post-preflight concurrent deletion or universal rollback across late
  SDK ref/reflog failures.

This implements local commit mechanics under ADR 0004, not a new history
store, `.cade` format, save UI or app/job API.

#### Local branch creation and ancestry — Stage A

The public methods are implemented in the private
[`branches.rs`](../crates/cade-git/src/branches.rs) module on existing
`ProjectRepo`, using existing `GitError` and SDK types:

```rust
pub fn create_branch(
    &self,
    reference: &str,
    target: gix::ObjectId,
    committer: gix::actor::SignatureRef<'_>,
) -> Result<(), GitError>;

pub fn merge_base(
    &self,
    one: gix::ObjectId,
    two: gix::ObjectId,
) -> Result<Option<gix::ObjectId>, GitError>;
```

- **Explicit immutable inputs:** branch creation takes a valid full local
  `refs/heads/...` name and an available, decodable commit target, not a tag to
  peel or content inferred from HEAD/index/worktree. Existing private name and
  identity validation is reused. The supplied reflog actor must validate and
  round-trip without normalization; identity is not fabricated from global
  configuration.
- **Strict creation, no dereferencing:** known invalid inputs and existing
  loose, packed or symbolic refs are rejected before locks. An identical
  existing target is also a denial. Publication retains SDK `MustNotExist`
  with `with_deref(false)`; the SDK's identical-target no-op is detected and
  rejected rather than reported as creation. Competing creators cannot
  overwrite each other or both report successful creation.
- **Preservation and explicit retry:** only the intended branch/reflog creation
  is authorized; HEAD attachment, worktree, index, config, objects and other
  refs are preserved. No checkout, hooks, external filters, subprocess Git or
  network operation is invoked. Locks, permission obstructions and invalid
  requests yield actionable `GitError::Git`; corrected requests can be retried
  explicitly on the same handle using freshly reopened metadata. There is no
  automatic retry, force update or destructive repair. These boundaries do not
  promise universal rollback for late multi-file SDK failures.
- **Complete, valid ancestry:** both explicit OIDs and all reachable parent
  commits must be available and decodable, even for equal inputs. Missing,
  corrupt or noncommit graph objects are errors, not fabricated absence.
  Valid unrelated histories return typed `Ok(None)` from SDK absence semantics,
  without error-string matching. Otherwise the result is one SDK-selected best
  common ancestor: multiple best bases are not rejected, sorted by a new rule
  or claimed to be unique.
- **Read-only queries, independent of HEAD:** merge-base changes no refs, index,
  worktree files or config. Both methods support bare repositories and detached
  or unborn HEAD when their explicit commit inputs meet the requirements.

Stage A uses gix 0.90.0; the feature-specific git2 dependency does not
replace these implementations or add a wrapper, error variant or app IPC.

#### Existing-worktree checkout — Stage B

**Decision and rationale.** Under #50's explicit feature-specific fallback
allowance, checkout uses exact `git2 =0.21.0` with default features disabled.
The inspected gix 0.90.0 `Worktree::checkout` performs initial linked-worktree
checkout, rejecting main worktrees and existing indexes; it does not supply
this existing-worktree branch switch. This is a narrow SDK capability gap,
not a replacement for gitoxide or a change to accepted ADR 0004. The later
#600 upload fallback enables workspace git2 HTTPS and vendored-libgit2;
checkout itself performs no transport operation, and SSH is not enabled.
Checkout uses SDK APIs, not
subprocess Git, shell commands or a custom lock protocol.

The public method in [`checkout.rs`](../crates/cade-git/src/checkout.rs) reuses
`ProjectRepo`, `GitError` and the existing SDK actor type:

```rust
pub fn checkout(
    &self,
    reference: &str,
    committer: gix::actor::SignatureRef<'_>,
) -> Result<(), GitError>;
```

- **Explicit target and fresh validation:** switching requires a worktree and
  an existing direct full `refs/heads/...` branch targeting a decodable commit.
  Bare repositories, missing/symbolic targets and invalid target/baseline
  objects are denied. Detached and unborn HEAD are supported. Configuration
  and index are read freshly, and supplied actor metadata is checked for
  lossless SDK representation rather than replaced with global identity.
  Native paths are not converted to lossy strings.
- **Other-worktree branch occupancy:** SDK enumeration includes the main and
  linked/sibling worktrees. Deny a switch when another worktree's symbolic
  HEAD names the target branch; exclude the caller's own canonical metadata
  identity and a bare main repository without a worktree. Unreadable
  registrations/HEAD fail closed. Inspect before acquiring guards and rescan
  under the held target-branch guard immediately before native mutation.
  The caller's already-current branch retains its existing validated no-op.
  Switch or detach the owning worktree before explicitly retrying a denial.
  This detects observed occupancy; it does not lock every worktree's HEAD
  or guarantee global atomicity against noncooperating writers.
- **Private SDK index, continuous real-index exclusion:** acquire the gix
  real-index update lock before reading its bytes. Load an operation-owned
  temporary index through libgit2 and attach it with `Repository::set_index`.
  Safe checkout updates that private index; its SDK-written bytes are prepared
  in the held real-index lock, without a handwritten index codec or a second
  attempt by libgit2 to lock the real index. A born HEAD requires its existing
  index so missing bytes cannot silently discard staged selections.
- **Two reference transactions and publication order:** one git2 transaction
  locks and publishes HEAD; a separate, uncommitted transaction holds
  deduplicated current/target branch guards. Revalidate initial reference
  observations under those locks. After successful SDK checkout and index-byte
  preparation, publish HEAD **before** committing the real index. Keep branch
  guards alive through both publications and the real-index lock continuously
  held through final index publication. Identical-value reference edits are
  not used as guards.
- **Safe overlap checks and latched veto:** SDK tree/index comparisons reject
  overlapping staged selections, including ancestor/descendant paths. Safe
  checkout disables ignored-file overwrite, conflict allowance, untracked or
  ignored removal and recreation of missing files. Prewrite conflict
  notifications latch a veto that prevents HEAD/index publication even if the
  binding returns `Ok`. `dry_run()` is not used as a conflict-safety proof.
  Unrelated safe staged, unstaged, untracked and ignored edits are preserved;
  no force, reset or stash is performed.
- **Explicit unsafe-state denials:** configured executable clean/smudge/process
  filters, gitlinks/submodules, nested repository traversal, unmerged indexes
  and unresolved merge/rebase/cherry-pick/revert/sequencer operations are
  rejected, not executed or silently ignored. Known metadata, reference-lock,
  index-lock and content obstructions are checked before content changes.
- **Partial failure and corrected retry:** this is not a globally atomic
  transaction. HEAD publication releases its own lock; branch/index guards
  do not protect against noncooperating direct file or HEAD writes. Late SDK
  object-read or filesystem failures can leave partial worktree changes
  before HEAD/index publication. A separate final index
  publication failure can leave changed HEAD/worktree with the old real index.
  Errors report possible partial state, never success or universal rollback.
  Cleanup is limited to operation-owned resources: user files are not
  automatically restored. Inspect HEAD, index and worktree, explicitly correct
  the obstruction/state, then retry; an unchanged partial-state retry may
  safely refuse.

**Native dependency boundary.** The lockfile resolves `git2 0.21.0`,
`libgit2-sys 0.18.8+1.9.7` and `libz-sys 1.1.30`. The captured macOS build uses
vendored, statically linked native libgit2 1.9.7 and system zlib, not a pure-Rust
checkout implementation. For #600, the manifest explicitly enables
`vendored-libgit2` and `https`, retaining default features false and the exact
git2 version. Delivery must verify the bundled selection: an owner-controlled
`LIBGIT2_NO_VENDOR` override can defeat it and must fail the provenance gate,
not silently substitute a system library. [NOTICE](../NOTICE) distinguishes Rust
binding licenses from native and bundled-component notices; it is an inventory,
not blanket license or release clearance.

This synchronous crate method adds no app consumer, IPC, serialized type,
`.cade` format or policy schema. Progress, cancellation and #602's original
500-ms/write-safety criteria are not delivered by this implementation.
Save/run integration, full push acceptance, configured LFS and host/platform
acceptance retain their separate obligations. Clone/fetch remain on gix;
the later push upload fallback does not alter checkout's publication order.

#### Explicit clone and fetch — #599

**Status and ownership.** Implemented under the architect-approved contract,
within ADRs 0004 and 0005, not a new architecture decision. Git mechanics stay
in `cade-git`; network construction and destination/credential validation go
through `cade-github`, with `cade-policy` alone deciding permission. Dependency
direction is `cade-git -> cade-github -> cade-policy`; Git also directly uses
the existing policy and secret types. No reverse Git dependency, new auth
framework, app caller, IPC, `.cade` or policy schema is introduced. Existing
GitHub API/auth and app feedback consumers are unchanged but require regression
verification because their crate dependency changes.

The public methods in
[`cade-github/transport.rs`](../crates/cade-github/src/transport.rs) and
[`cade-git/transport.rs`](../crates/cade-git/src/transport.rs) reuse existing
`Host`, `ProjectRepo`, `GitError` and SDK types:

```rust
impl Host {
    pub fn validate_git_remote(
        &self,
        remote: &url::Url,
        token: &octocrab::secrecy::SecretString,
        policy: Option<&cade_policy::Policy>,
    ) -> Result<(), GitHubError>;

    pub fn git_connection<'remote, 'repo>(
        &self,
        remote: &'remote gix::Remote<'repo>,
        token: &octocrab::secrecy::SecretString,
        policy: Option<&cade_policy::Policy>,
    ) -> Result<
        gix::remote::Connection<
            'remote, 'static, 'repo,
            Box<dyn gix::protocol::transport::client::blocking_io::Transport + Send>,
        >,
        GitHubError,
    >;
}

impl ProjectRepo {
    pub fn clone<P>(
        destination: &std::path::Path,
        remote: &url::Url,
        host: &cade_github::Host,
        token: &octocrab::secrecy::SecretString,
        policy: Option<&cade_policy::Policy>,
        progress: P,
        interrupt: &std::sync::atomic::AtomicBool,
    ) -> Result<Self, GitError>
    where
        P: gix::NestedProgress,
        P::SubProgress: 'static;

    pub fn fetch<P>(
        &self,
        remote: &url::Url,
        host: &cade_github::Host,
        token: &octocrab::secrecy::SecretString,
        policy: Option<&cade_policy::Policy>,
        progress: P,
        interrupt: &std::sync::atomic::AtomicBool,
    ) -> Result<gix::remote::fetch::Outcome, GitError>
    where
        P: gix::NestedProgress,
        P::SubProgress: 'static;
}
```

- **Shared preflight before side effects:** `Host::validate_git_remote` is
  zero-I/O and shared by clone before destination creation, fetch before
  reopening, and `git_connection` before transport construction. The borrowed
  SDK Remote/Connection needs a repository; clone does not create a temporary
  repository merely to validate permission. A supplied trustworthy policy must
  return `Allow` for `Action::AgentNetwork`; missing policy, `Deny` and `Ask`
  are not permission. No default policy is substituted.
- **Explicit host and repository:** validate Dotcom, GHE.com tenant labels and
  configured GHES HTTPS origins, effective ports and base paths. Require exactly
  owner/repository beneath the selected base, with optional `.git` suffix;
  components use ASCII letters, digits, `-`, `_` or `.`, excluding empty, `.`
  and `..`. Reject userinfo, query/fragment, encoded components, endpoint
  suffixes, other origins/tenants and non-HTTPS schemes, including SSH/file/
  helpers. No discovery, repository creation or unsolicited endpoint is added.
- **Credential provenance remains a caller obligation:** a nonempty
  `SecretString` must already belong to the selected Host. Validation cannot
  prove that association or remote authorization. No implicit global keychain
  lookup, reassignment to a new host, refresh or new storage is implemented;
  actual #262/#263 selected-host/keychain integration remains upstream work.
  The transient SDK identity uses `x-access-token` and the supplied secret.
  Its SDK-owned string copy is not a new redacted/zeroizing credential store.
  The explicit callback returns no helper fallback and has no Store/Erase
  effects; no credentials are persisted or logged.
- **Explicit connection options:** pinned gix HTTP/reqwest transport uses
  protocol V2 and native TLS with certificate and hostname validation enabled.
  Options are installed on the Connection, not merely its factory. Redirects,
  proxies (including environment/config proxies), extra headers and verbose
  tracing are disabled; no repository CA override is supplied. Direct factory
  calls reject enabled/invalid packet tracing; clone/fetch use an isolated
  transient tracing-off override. `remote_at_without_url_rewrite` selects the
  supplied URL. Proxy integration is unsupported. Native-trust selection is
  implemented configuration, not executed OS-trust or hosted-TLS acceptance.
- **Fixed diagnostics:** `GitHubError::GitTransport(&'static str)` extends the
  existing enum; empty tokens retain `NotSignedIn`. Git maps these into existing
  `GitError::Git` with fixed corrective guidance. New transport paths do not
  forward raw SDK/provider Display/Debug, URLs, headers or server-controlled
  error text. Failures identify possible retained state, not false success.
- **Tracking-only fetch:** reopen isolated metadata/configuration per request;
  ignore global/system/environment config and includes, reject nondefault
  namespaces and configured executable clean/process/smudge filters. Use only
  `+refs/heads/*:refs/remotes/origin/*`, with tags disabled. Inspect mappings,
  reject symbolic source/observed tracking targets and invalid namespaces,
  check rejected/type-changing/dereferencing updates, and reread direct
  advertised OIDs before success. A top-level SDK `Ok` alone is insufficient.
  Fetch preserves local commits/branches, HEAD, index, worktree, config and
  existing reflogs; reflog suppression is transient, with no invented actor.
  No checkout, merge, reset, pruning or automatic retry occurs. Failed transfers
  may still leave downloaded objects or partially published tracking refs.
- **Absent-destination clone:** atomically claim one absent directory with an
  existing writable parent; reject occupied directories (even empty), files
  and symlinks. Lower-level SDK init/fetch/ref/index/worktree composition avoids
  high-level clone's generic connector and destructive drop cleanup. There is
  no recursive cleanup after errors or guarantee excluding arbitrary external
  writers. Persist only credential-free origin URL, tracking refspec and tag
  policy, not a resolved configuration containing transient credentials/options.
- **Actual advertised HEAD and initial checkout:** require a symbolic remote
  HEAD naming a fetched valid branch with its matching object; do not guess
  an unrelated default. Empty/unborn, missing/dangling/direct HEAD and corrupt/
  missing objects yield actionable retained-state errors, not fabricated
  commits or empty-clone success. Validate the selected tree/blobs and reject
  gitlinks before checkout writes. Use gix index/worktree checkout with exclusive
  initial file creation; check collisions, errors, interruption, delayed paths
  and completed file count, then publish the actual index. Only afterward
  publish the initial local branch/born HEAD with SDK expected-value,
  no-dereference edits. Competing local refs are not reset or overwritten.
  This order applies only to initial clone: #598's existing-worktree git2
  switch still publishes HEAD before its final real index, and its
  born-HEAD/missing-index guard remains unchanged.
- **Partial failure and explicit recovery:** a claim/init failure retains any
  partial destination; once initialized, errors retain metadata, objects,
  published refs and partial/user worktree content. An earlier collision or
  index-write failure does not publish clone's local branch/born HEAD: absent
  an external writer, HEAD remains unborn, with downloaded objects/tracking
  refs and foreign/partial worktree bytes retained. Later local-reference
  publication failure can leave the worktree and index already written,
  alongside competing or partially published refs. Inspect first. If usable,
  reopen with `ProjectRepo::open`, correct the cause, explicitly `fetch`, then
  select a fetched commit and `create_branch` with a valid unused full local
  ref and supplied actor if needed; explicitly `checkout` the selected existing
  local branch with a supplied actor after resolving obstructions. Reconcile
  already-published branches rather than recreating or overwriting them.
  Retrying clone over the retained occupied destination is not recovery.
  Checkout/index/reference failures are not rolled back or automatically
  repaired; no synthetic index or bypass of the missing-index guard is used.
  Recovery remains subject to normal safety checks and owner correction, not
  a guarantee that every retained state can immediately be checked out.
- **Bounded caller progress:** the supplied `NestedProgress` receives fixed
  client-owned stage names/messages and actual numeric outcomes. Advertisement
  reports the returned remote-ref count. Pack receive reports the indexed-pack
  object count, or zero when no pack is received, not unique newly downloaded
  objects or wire bytes. Tracking verification reports validated mapping count
  after validation and interruption checks, including unchanged refs rather
  than only newly written ones. Raw SDK/server task names, sideband messages
  and error text from advertisement/receive go to SDK `Discard`, not the caller.
  Initial SDK checkout keeps live file/byte counters through its numeric-only
  `Count` interface, which cannot emit task names/messages; its surrounding
  start/completion messages are client-owned. Advertisement/pack totals remain
  unknown; only known checkout-entry/mapping counts supply maxima. No estimated
  percentages or invented totals are supplied. Transfer completion follows
  validation; checkout-stage completion does not imply that subsequent local
  branch/HEAD publication succeeded.
- **Separate unsupported guarantees:** no hooks, production Git/SSH subprocess,
  executable filters, automatic submodules or LFS execution. The original
  caller interrupt flag reaches receive and initial checkout stages, but
  the pinned handshake/ls-refs has no interrupt argument; the flag is checked
  when it returns. Neither 500-ms cancellation, all-or-nothing writes nor
  universal filesystem rollback is proven. The bounded #600 push implementation
  below does not discharge those obligations; #600, #601 configured LFS and
  #602 typed cancellable jobs retain their original scope and criteria.

Root gix features now include `blocking-http-transport-reqwest-native-tls` and
`worktree-mutation`. Existing resolved SDK versions, including gix 0.90.0,
remain unchanged; the ten added feature-related packages and native-source
boundary are listed in [NOTICE](../NOTICE). This is not a new ADR, SDK upgrade,
git2 clone/fetch path, installer change or release approval.

#### Explicit single-branch push — #600

**Implemented contract, not full acceptance.** Under the approved #600 contract
and #50's feature-specific fallback allowance, gix 0.90.0 performs validation
and fresh remote observation, while pinned `git2 =0.21.0` performs upload.
The inspected gix SDK lacks the needed upload operation; no handwritten Git
protocol replaces it. ADRs 0003/0004/0005 remain unchanged, including Copilot
SDK `=1.0.26`, branch-plus-PR shipping and sole `cade-policy` authority.
Only two public methods are added, using existing SDK/error/progress types:

```rust
impl ProjectRepo {
    pub fn push<P>(
        &self,
        reference: &str,
        expected: gix::ObjectId,
        destination: &str,
        remote: &url::Url,
        host: &cade_github::Host,
        token: &octocrab::secrecy::SecretString,
        policy: Option<&cade_policy::Policy>,
        progress: P,
        interrupt: &std::sync::atomic::AtomicBool,
    ) -> Result<gix::ObjectId, GitError>
    where
        P: gix::NestedProgress;
}

impl Host {
    pub fn git_push_remote<'repo, 'auth>(
        &'auth self,
        repository: &'repo git2::Repository,
        remote: &'auth url::Url,
        token: &'auth octocrab::secrecy::SecretString,
        policy: Option<&'auth cade_policy::Policy>,
        callbacks: git2::RemoteCallbacks<'auth>,
        interrupt: &'auth std::sync::atomic::AtomicBool,
    ) -> Result<(git2::Remote<'repo>, git2::PushOptions<'auth>), GitHubError>;
}
```

The methods live in the existing private `transport.rs` modules of
`cade-git` and `cade-github`. No new public wrapper, framework, error code,
policy action/schema, serialized format, IPC, capability or generated binding
is introduced.

- **Preflight and explicit local selection:** shared zero-I/O Host validation
  precedes project preparation. Both names must be full `refs/heads/...`;
  neither HEAD, revision syntax nor a user-supplied refspec selects content.
  Fresh isolated gix metadata must expose a direct local branch equal to
  `expected`, a decodable commit, supported object format and default namespace.
  Shared executable-filter rejection remains enforced. Object preflight checks
  reachable commit/tree/blob kinds, checksums and decoding through SDK traversal,
  respecting shallow boundaries; it never substitutes index/worktree content,
  follows gitlinks or supplies LFS payloads. Object-graph edge-case proof remains
  qualified below; this is not a universal object-integrity certification.
- **Guard and immutable wire source:** a no-edit SDK transaction locks only the
  selected source branch. Fresh under-lock validation rejects changed, missing
  or symbolic tips. The guard stays held through observation/upload/return and
  drops without committing or setting a target; it neither locks HEAD/index
  nor deletes a foreign marker. One parsed/serialized
  `<full expected OID>:<full destination>` refspec, without `+`, binds the upload.
  Exactly one negotiated update must match that source, destination and nonzero
  new OID. This is non-force, not force-with-lease or a distributed transaction.
- **Local preservation:** no staging, commit, checkout, tracking-ref/reflog
  publication, persistent config write, hook, executable filter, submodule
  traversal, LFS execution or automatic retry/rollback is part of push.
  Dirty/conflicted index and worktree do not select content. Bare, detached
  and unborn HEAD work when the explicit branch/commit and metadata are valid.
  `.cade` blobs are opaque, including old or malformed document contents;
  no document migration/openability or LFS completeness is implied.
- **Host-owned construction:** on an operation-private git2 handle, Host
  installs an empty in-memory config and an anonymous remote with exact
  selected URL/push URL checks. It preserves mechanics callbacks while replacing
  credentials, certificate and sideband callbacks. Credentials permit one
  selected-URL `x-access-token` user/password offer, rechecking policy and
  interruption; helpers, prompts, fallback identity and persistence are absent.
  Native `CertificatePassthrough`, never accept-all, replaces any supplied
  certificate override. Sideband text is discarded. Returned options specify
  no proxy, no redirects, no custom headers and no remote push options.
  These mutable SDK options must be used unchanged; they are not an opaque
  security capability. Selected-host token provenance is still the caller's
  responsibility, not proven by a nonempty token or the global keychain entry.
- **Platform boundary:** Windows and other unsupported platforms return a
  fixed unsupported-secure-push error before networking, including before the
  gix pre-query. This is a missing capability, not Windows acceptance.
  macOS/Linux construction paths and configured options do not prove native
  trust, process-global TLS/proxy/trace isolation, redirects or on-wire auth.
  No owner OS settings are changed to make those proofs pass.
- **Fresh outcomes:** each pre/post-query uses a new isolated gix connection
  through Host, without URL rewriting, writable refspecs, fetch or cached
  git2 ref lists. Exactly one direct destination advertisement is usable;
  duplicates or symbolic/ambiguous advertisements fail. Already-at-expected
  pre-query returns the OID without upload. Otherwise one upload is attempted,
  then a fresh observation unless interrupted. Normal success requires one valid
  negotiation, exactly one accepted destination report, top-level SDK success
  and fresh equality. Rejection, missing/duplicate/unexpected reports and
  negotiation failure remain errors even if a later observation equals expected.
  A top-level upload error stays an error even if the intended tip is observed.
  Failed verification or interruption after entering push can mean remote
  effects exist; error never guarantees that nothing changed remotely.
- **Explicit recovery:** inspect remote state, correct credentials/policy or
  connectivity, then make a newly authorized request. Its fresh pre-query may
  observe completion without retransmission. A changed local branch requires
  explicit reselection, never silent substitution. Remote hooks/object transfer
  may have effects, and refs may change immediately after observation. There is
  no automatic upload retry, remote undo, attribution of who moved a ref or
  universal filesystem-preservation claim against noncooperating writers.
- **Policy and progress:** explicit effective policy must Allow
  `Action::AgentNetwork`; `network: On` does not authorize direct `ShipByPush`.
  #140/#141 still own user intent, preflight and branch-plus-PR shipping; push
  neither opens a PR nor completes Ship. Progress exposes fixed client stages
  and numeric pack/object/byte/verified-ref counts, not raw server diagnostics.
  Original interruption checks and returning callbacks are retained, but
  `pack_progress` and `push_transfer_progress` return `()` and cannot cancel I/O.
  Blocked sockets/handshake can outlast the flag. Strict 500-ms stop/no-partial-
  writes remains an original #46/#602 obligation, not a primitive guarantee.
  Future job consumers must keep this synchronous operation off the UI thread.

**Dependency and consumer impact.** Root git2 keeps default features false and
adds only `https` and `vendored-libgit2`; `cade-github` adds `git2.workspace`.
The existing checkout linkage therefore changes through feature unification,
as does the app's cade-github dependency graph. No SSH, SDK upgrade, new
installer or vendored-OpenSSL requirement is introduced. [NOTICE](../NOTICE)
records the extra openssl-probe resolution, native backend distinctions and
distribution-review limits.

Regression consumers include cade-git's existing local/checkout and clone/fetch
tests, cade-github's validator/gix factory/auth/API/PR tests, unchanged
cade-policy decisions, cade-app's existing feedback/auth consumer, and indirect
cade-feedback/UI/generated-binding behavior. The app is not a push caller.
Future #139 selection, #140/#141 Ship/PR, #465 offline queue, #601 LFS with
#466 configuration, and #602/#36/#46/#54 jobs/IPC must integrate their own
approved contracts. #50's combined workflows retain content, ancestry and
run-trailer obligations. No consumer integration is inferred from crate tests.

**Historical developer checkpoint — October 27, 2026, 22:36:49 −07:00.**
`git600-implementation/batch16/handoff.json` and `plan-ledger.json` identify
297 paths at tree `19a6cef843bf67479c3674f9fa3728c9a7ea31d9`, parent
`0fb8fcc4968f8553bcc6fcf29a9ab8f834745f16`, not a #600 commit.
Four Rust checkpoints passed on Darwin: fmt, workspace Clippy, 853 workspace
tests and app build. Canonical row categories are mutually exclusive:
10 red/green + 27 initial-pass regressions + 4 partial + 3 untouched = 44;
zero rows had independent acceptance. Initial passes and retrospective mutants
are not red-first credit. Rows 43/44 exercise a local clone/commit/branch/push
workflow and network-versus-direct-Ship distinction, not hosted workflow/PR
completion. Row 40's new Windows-cfg test was neither compiled nor executed on
Darwin; test presence is not PASS. The receipt did not include full twelve-gate,
native #600 CI or docs-QA acceptance.

**Delivery and original-scope obligations.** Full twelve-gate execution,
independent behavioral/regression/docs QA, exact successor source/binary/native
bindings and applicable exact-head hosted checks are mandatory for delivery.
The dated checkpoint above must not be used as a live gate dashboard or
transferred to a documentation successor. Remaining developer qualifications
include row 10 malformed/shallow/object-graph cases and guard/default-argument/
cancellation mutation matrices; the batch7 zero-OID survivor remains a QA
qualification. The earlier type-delete survivor was caught by batch15, not
erased or credited as a new behavior red. Partial rows 35/36/39 retain
process-isolation, authentication-composition and native logging limits;
rows 38/41/42 retain certificate/TLS/proxy/redirect proof requirements.
True dropped acknowledgment, full malformed-wire and native offline/failure
matrices remain separate from local injected failures.

Full original #600/#50 acceptance still requires GitHub.com/GHES intended
ref/content and combined workflow proof, hosted branch-protection/permission
rejections, actual selected-host OS-keychain/missing-entry/revoked-credential
behavior, native TLS/trust/proxy/redirect security and required Windows/Linux
execution. Full original #598/#599/#50 caveats also remain: initial-clone fault
cases, native extras, real host/authentication, save/run/app integration,
configured LFS (#601) and cancellable IPC/jobs (#602) are not discharged by
bounded repairs or this push implementation. S1 Low #106 remains unfixed.
No closing reference or issue completion follows; use Refs #600 and #50.

### Policy failure boundaries

**Status:** accepted bounded contract; #537, #539. Preserves
[ADR 0003](adr/0003-copilot-sdk-and-byok.md) and
[ADR 0005](adr/0005-policy-as-code.md).

With `budgets.monthly_tokens` configured, arithmetic overflow in `used + next`
denies spending, even when the cap is `u64::MAX`. A representable sum equal to the
cap allows; a sum above it denies. Behavior without a configured budget is
unchanged. No policy schema or public API changes.

An unreadable/poisoned organization-policy lock rejects permission requests
without prompting or approving, with or without managed approval. It does not
clear poison or substitute user consent for a policy decision. New recovery
mechanisms are out of scope; trustworthy access must precede evaluation of any
fresh request. Bounded denial approval does not certify shipping recovery or
end-to-end token-budget consumers.

### Generator compatibility

**Status:** accepted bounded contract; #547, #548, #561 (DDL), #552, #563
(React); #550's narrow parser correction accepted, broader choice open.

Newly generated overlong DDL constraint names use a readable prefix and a
deterministic suffix derived from the full constraint identity. Ordinary names
remain unchanged. Creation and applicable migration up/down paths use the same
identity, removing the matching constraint before its column where required.
This covers MySQL foreign keys, SQL Server defaults and SQL Server/Oracle enum
CHECK constraints.

Existing emitted scripts and deployed constraints are not automatically renamed
or rewritten. Reconcile names explicitly before using newly generated migration
output against an older deployment. The DDL public/serialized model is unchanged;
this is an emitted-output correction, not a format migration.

React generation preserves literal values as valid JSX, accepts supported
Unicode/underscore/dollar/dotted identifiers with matching imports, and retains
typed rejection of hostile names. IaC retains literal escaping and quotes
parser-significant `for` keys without changing ordinary goldens. This does not
resolve the all-key quoting choice in [OQ-550](OPEN-QUESTIONS.md#oq-550--terraform-map-key-quoting).

### Feedback contract

**Status:** accepted bounded contract; #309, #310, #311, #558, #581 under
[ADR 0006](adr/0006-feedback-as-github-issues.md). Architect-approved details
under the product owner's remediation authorization; not a new payload wrapper.

`Feedback.answers` is a canonical form-ID-to-string `BTreeMap`. The deserialization
adapter accepts legacy heading/text arrays, mapping known headings to IDs.
Unknown, foreign-form and duplicate answers cannot satisfy required fields or
inject headings; missing or whitespace-only required values fail validation.
`Answer` and the other public domain names remain available. This preserves
legacy wire input, not Rust source compatibility for old `Vec<Answer>`
constructors.

| Form | Canonical answer IDs | Required answers | Exact labels |
|---|---|---|---|
| Bug | `what-happened`, `steps`, `expected`, `actual`, `output`, `area`, `severity` | `what-happened`, `steps` | `bug`, `needs-triage`, `type:bug` |
| Feature | `problem`, `proposal`, `alternatives`, `output`, `beneficiaries`, `contribute` | `problem`, `proposal` | `enhancement`, `needs-triage`, `type:feature` |
| Comment | `where`, `comment`, `output`, `reply` | `comment` | `feedback`, `type:comment` |

YAML issue forms own metadata; Rust types generate TypeScript declarations with
ts-rs, and generated form metadata supplies the existing UI facade. Do not
hand-copy either contract. This bounded use of #36 does not complete all IPC
binding work. Output is multi-select, encoded with comma-space in YAML option
order; Rust validates and canonicalizes selections. Empty means unanswered.
Beneficiaries retain `individual`, `team`, `admin/security` choices from YAML;
contribution/reply use optional Yes/No selects. Area/severity remain free text.

Environment retains `app_version`, `os`, `gpu`, `policy_version`, logs and per-item
opt-out; screenshots use the existing `screenshot_url` HTTPS/redaction path.
The hosted bug form's old `version` ID migrates to existing `app_version`, with
legacy `version` input accepted where applicable. This named correction supersedes
the earlier instruction to leave that discrepant ID untouched. No feedback
payload version existed, and no `.cade` or policy format changes.

The existing `preview_feedback(Feedback)` and `submit_feedback(Feedback)` IPC
signatures and `IssueDraft` shape remain unchanged. Only a successful Rust preview
for the exact current content authorizes submission; absent, pending, failed or
stale previews do not. Submission captures that successfully previewed input
snapshot; in-flight edits cannot substitute different content. Content-keyed
requests and stale-response rejection remain. Failures retain entries for retry.
Success immediately focuses the existing “Issue opened” heading inside the
dialog, preserving focus trapping, Escape and opener restoration.

The approved field, unavailable-data, retry and failure copy is resolved in the
feedback requirements, not awaiting another wording decision. Consumers include
Rust callers/migration fixtures, native preview/submit, generated bindings and
metadata, the dialog, e2e IPC mocks, hosted forms/labels and `cade-github` submission.
The [contributor guidance](../CONTRIBUTING.md#maintaining-feedback-contracts)
records their maintenance obligations. Local parity does not prove hosted
rendering/publication; browser focus does not prove native accessibility.

## Acceptance evidence boundaries

The local Git fixtures cover open/init/status, actual stored initial and
parented commits, branch creation, explicit-commit ancestry and checkout.
The [commit probes](../crates/cade-git/tests/repo/commit.rs)
inspect selected trees, parents, stored bytes and trailers, with separate
validation, preservation, lock, stale-tip and retry cases.
The [branch probes](../crates/cade-git/tests/repo/branches.rs) cover strict
creation, supplied actors, competing creators and corrected retries;
[ancestry probes](../crates/cade-git/tests/repo/merge_base.rs) cover unique,
multiple-best, unrelated and invalid/incomplete histories with preservation.
The [checkout probes](../crates/cade-git/tests/repo/checkout.rs) exercise
content preservation, unsafe-state denials, reference/index locks, actor
validation and corrected retries. Git CLI use in these tests builds fixtures
or independently inspects results; it is not the production implementation.
Private `cfg(test)` checkout observers support boundary probes; they are not
public API or runtime telemetry. Native object-read failure after an earlier
worktree write and late real-index publication failure are distinct proof
boundaries, not interchangeable evidence of an OS worktree-write failure.
Stage A received bounded independent QA approval and exact-commit core gates
at `bb8460ac012c668310fecb85d7b27adc7ea69fc0`. The later checkout disposition
accepted bounded macOS implementation and README/ARCHITECTURE/NOTICE at
`ad522fe718182db90861a6c39197b9dab7ca79dc`, using saved shared execution of all
three applicable core gates and independent public-API probes. This is
independent QA acceptance of shared evidence, not a claim that QA reran the
gates. At that disposition, Windows/Linux and two Linux-only fixtures were
unexecuted; late object/index-publication evidence does not certify OS
worktree-write faults.

**Historical #598 disposition — October 27, 2026; readback after 18:19:12 −07:00:** the 12-file test-only repair
is committed at `ed093023794b791057bbe308d2c86fd0f173af66`, tree
`e9bb58276ce84dce751cad214a3e7b9f6a8eddbf`. Developer and independent QA
execution passed 742 workspace, 12 app, 134 UI and 104 E2E tests, with
20 library and 239 repository hostile-environment tests. Counts are separate
selectors, not a summed unique-test total. B1/B2 proof gaps are closed and all
seven LOCAL criteria are accepted; those proof/setup gaps were not production
defects. The parent receipt at 18:12:22 −07:00 records run `40611212652`
Crates and Apps on Linux/macOS/Windows, UI, GHAS, GitGuardian and instructions
SUCCESS. CodeQL was NOT_TRIGGERED on the unprotected nondefault base, not PASS.
PR #614 is READY; actual head/base/body readback was completed after
18:19:12 −07:00, as supplied in the current handoff. These results do not
certify the final propagated #599 tree or unexecuted native extras, or establish
all original #598/#50 criteria.

External SDK capability experiments,
developer red/green results and focused checks do not substitute for production
QA and exact-candidate workspace gates. Linux-only raw-byte fixtures require
actual Linux execution, not a pass inferred from macOS exclusion.
Local repository evidence does not establish actual save/run integration,
hosted GitHub/GHES authentication, native extras, hosted transport/LFS/jobs,
all-platform behavior or release readiness. In particular, nonroot macOS
permission fixtures do not certify Linux permissions or Windows ACL behavior.
Independent QA and exact-commit full-workspace gate evidence must be assessed
separately; earlier gates do not certify later changes. Full execution logs,
source-hash inventories and per-test counts remain in the saved evidence;
the bounded disposition below is not a live gate dashboard.

For #599, public preflight/factory denial and connection-configuration tests
are distinct from private SDK pipe fixtures. The latter share the production
receive/ref/checkout composition and exercise actual synthetic protocol,
objects, tracking refs, advertised HEAD, preservation and explicit recovery;
they bypass the public HTTPS connection and do not prove hosted authentication
or TLS. The earlier October 27 fixture recheck recorded 46 new cases and 252
existing nonnetwork regressions passing on its recorded source. Its last
fifteen failures were fixture-path corrections, not fifteen product bugs;
27 original assertion reds and 19 initially-green controls remain distinct.
That source's formatting/compilation passes and unused-import Clippy failure
are historical, not verification of the later fix.

Subsequent independent execution-04 ran ten probes: eight passed, two failed
on caller-visible server text and retained-clone collision recovery; six
selected connection regressions passed. The two failures were reproduced as
persistent assertion reds, not setup failures. The October 27, 2026,
02:39:06–02:43:19 −07:00 scoped receipt for the patched uncommitted source
over `ad522fe` records:

| Scoped selector | Actual result |
|---|---|
| Persistent sideband diagnostic isolation | 1 passed after its preserved assertion red |
| Persistent collision recovery | 1 passed after its preserved assertion red |
| Supplied progress, unchanged assertions | 1 passed |
| Clone boundaries | 9 passed |
| Fetch boundaries | 7 passed |
| Transport integration | 10 passed |
| Existing checkout state | 25 passed |
| Existing checkout content | 13 passed |
| Existing checkout denials | 22 passed |
| Formatting check | Passed, exit 0 |
| Focused Clippy | Failed, exit 101; compilation/checking output truncated |

Selector counts overlap; they are not a unique-test total or workspace gates.
The truncated Clippy output and absence of dependent broad gates describe that
historical run, not current lint availability. The later repair receipt at
06:08:26 −07:00 records focused Clippy exit 0. The earlier SDK-download DNS
failure is also recovered history: the existing offline SDK cache was verified
and used for the guarded committed-source checks below, without an SDK download.

**Original #599 bounded disposition — October 27, 2026:** independent review of saved
execution accepted all 10 unchanged original probes and 6 configuration
regressions, including rewind/no-pack and affected GitHub/feedback checks.
Source verification and the bounded macOS implementation/behavioral-docs
recommendation passed (`qa-git599/reports/recheck1-final-disposition.md`).
Both original product findings are resolved; the 21 product paths are committed
at `ed53dba`. This is not a reviewer rerun or full issue acceptance.

The guarded committed-source receipt (`git599-guarded-gates-receipt.md`)
records workspace Clippy PASS (25.43 s), app build PASS (51.26 s), and app tests
PASS (12 tests). Four earlier formatting/UI build/typecheck/test successes
omitted the required guard, and Corepack reported an unexpected pnpm download
(`git599-commit-gate-handoff.md`); they are not guarded results. That receipt
did not establish full nine-gate acceptance. Earlier loopback/cache-copy denials are
resolved by existing explicit approval; they are not current permission blockers.
The user released the #599 pause on #598's required native CI success.
Test-owned localhost `127.0.0.1`/`::1` is approved; external network and real
credentials remain restricted.

**Propagated-tree evidence.** The October 27, 2026 final-integration developer
handoff records twelve passing local gates on tree
`69efec6c880e98991d78b5ae6ded520ef01526bb`, including 794 workspace, 12 app,
134 UI, 104 E2E and 3+3 generator passes. The 13 localhost API fixtures passed;
focused counts overlap workspace results. Only its `final2-` gate receipts
support that tree; the earlier `225c353c6c089f81cf5262f82a34ee95ba241901`
round is historical. These developer results are distinct from independent QA
and exact-head native acceptance, and do not certify any successor tree,
including a documentation-only change. Each candidate requires its own bound
evidence; the original 10+6 QA record is retained, not a fresh external-fixture rerun.
Thirteen earlier Wiremock API regressions failed socket setup under blanket
network denial, not assertions; that historical runner does not establish a
current failure or a pass on the final tree.

Initial-clone index/local-reference write-fault proof and prior mutation
evidence remain unexecuted; existing-checkout state greens do not prove
initial-clone write-fault behavior. Actual native OS trust, selected-host/
keychain integration, approved hosted GitHub.com/GHES repositories and
credentials, and original-scope Windows/Linux execution retain their separate
acceptance requirements. The later named #599 publication/local-QA/native
READY receipt is recorded above; it does not erase these original-scope limits.
No #599–#602 closure or #600 publication is implied.

Independent QA approved **13 bounded fixes**, not 13 issue closures. Evidence is
attached to the reviewed commits: policy `1ceb0837b6a2d9ae28f9156770e7b2cf2fb41f3c`
(2 fixes), generators `b2e34293aa9f91df6b35ca55d69c28191ebadaf1` (6), and feedback
`78616f59817d962636083e430da3efbe56d2a417` (5). Historical shared gates are not
reviewer-run full gates or tests of later documentation changes.

| Issues | Approved evidence and remaining issue-specific boundary |
|---|---|
| #537, #539 | Overflow denial and unavailable-policy rejection approved; broader budget-consumer coverage and shipping recovery are not prerequisites newly imposed on these bounded fixes. |
| #561 | Naming/schema/up/down agreement approved; original SQL Server 2022 executed up/down reproduction remains unverified. |
| #547 | Distinct/reversible emitted FKs approved; original MySQL 8 metadata/enforcement reproduction remains unverified. |
| #548, #552, #563 | All explicit code-generation criteria verified; generated SQL is not proof of database enforcement, and parser/compiler fixtures are not deployment certification. |
| #550 | Narrow `for` correction approved with actual Terraform parser/literal checks; full quoting choice awaits the product owner in OQ-550. |
| #309, #310 | Local form/payload/label/schema parity approved; authenticated hosted rendering/required enforcement, actual submitted labels and user-owned CI-lint wiring/execution remain unverified. |
| #311 | Local parity and existing contributor form guidance approved; hosted appearance and actual label application remain unverified. |
| #558 | Browser focus behavior approved; native webview/accessibility confirmation remains unexecuted. |
| #581 | Original request-count/order criteria and amended preview/snapshot/recovery behavior verified locally; no unrelated hosted-form, CI-lint or documentation blocker is added. |

The local remediation evidence is macOS-based; it is not Windows/Linux or native
assistive-technology certification. No blanket hosted/manual acceptance or
release approval follows. Separately, #342 remains blocked on available CADE
installers, and #344 still lacks monitoring evidence despite its approved contact.
Neither is an unresolved choice of contact or installation wording.

## Change history

Initial approval below was received from the product owner on
**2026-10-26 at 17:14:12.058 −07:00**. Earlier feedback implementation details were
architect resolutions under authorized remediation, not individually attributed
product-owner choices. Subsequent changes identify their own received times.

| Topic | Earlier state → current decision and rationale | Approval evidence | Status |
|---|---|---|---|
| Policy | Overflow saturation and unavailable-policy approval fallback → deny overflow and reject unreadable policy, preserving trusted policy authority. No new recovery mechanism. | “policyMAXoverflowdenyconfigured,no-budgetsame; poisonrejectnoapprovalnoclear newrecoveryoutofscope ADR0003/5.” | Accepted; bounded QA approved |
| DDL | Truncation-only overlong names → deterministic full-identity suffix for new generation, preserving ordinary output and existing deployments. | “Generatoroverlongdeterministicsuffix newgeneratedcreate/up/downconsistent; noautochangeexistingdeployedconstraints,noDDLserializedchange.” | Accepted; bounded QA approved |
| Feedback | Heading arrays and handwritten mirrors → keyed map with legacy adapter and generated declarations/metadata. Earlier briefs left labels/copy/version unresolved; the approved contract resolves them, including the named environment-ID correction. | “Feedbackcanonicalkeymap legacyshapeadapter/knownvalidation,Rust-derivedts-rs declarations,YAMLauthority,labelslegacy+requiredtype,version->app_versionmigration; actualsuccessfulcurrentpreview snapshot&focussuccess.” | Accepted; bounded QA approved |
| Git inventory (historical) | Unqualified intended diagram → explicit scaffold boundary while local status was undelivered. Superseded by the implemented local foundation below. | “Gitstatus NOTimplemented yet; don'tclaim” | Superseded |
| Git local foundation — 2026-10-26 at 18:14:08.215 −07:00 | Scaffold-only inventory → implemented #596 open/strict init/read-only status under the approved `remediation/git-local-contract-brief.md` and ADR 0004. The earlier inventory left commit creation undelivered; the commit entry below supersedes that boundary. | Product-owner handoff: “RecordimplementedGitlocalopen/init/status”; “independentQA/fullgatespending doNOTclaimoverallcomplete.” | Implemented; inventory extended below |
| Git local commits — 2026-10-26 at 19:10:45.332 −07:00 | Status-only inventory → implemented #597 explicit immutable-tree commits under the approved `remediation/git-commit-contract-brief.md`, checked against source. Reuse SDK input and strict atomic expected-tip publication; preserve caller bytes and unrelated state without mutable-index selection. ADR 0004 remains unchanged; local mechanics do not establish save/run integration or completion of #50. | Product-owner handoff: “existingSDKCommit input notcustomwrapper”; “callerfullrefs/heads ref+parents expectedtip strictatomic CAS noderef+SDKindexholdlock noindexrewrite”; “no allengineclaim.” | Implemented local contract; not an acceptance or closure ruling |
| Git branches/ancestry — handoff received 2026-10-26 at 20:20:14.247 −07:00 | Earlier inventory excluded all #598 operations → implemented Stage A `create_branch`/`merge_base` under `remediation/git-branches-contract-brief.md`, checked against source. At that handoff checkout was gated and QA outstanding; those historical boundaries are superseded below. Also corrected #597's missing-only preflight description to its delivered full observed-reference preflight, retaining race/rollback limits. | Earlier handoff: “Document only actual StageA capabilities and remaining checkout/transport/LFS/jobs/platform gaps.” “Accepted ADR0004 unchanged; #50 permits a narrow future fallback but it is not delivered.” | Historical inventory; authorization and acceptance boundaries updated below |
| Git checkout — authorization received 2026-10-26 at 21:39:43.371 −07:00; implementation handoff received 22:05:43.606 −07:00 | Stage A bounded QA and named-commit gates accepted; prospective fallback → actual checkout-only git2 adoption for the identified gix main-worktree switch gap. The updated Stage B decision supersedes the earlier index-before-HEAD proposal: publish HEAD, then real index, with separate branch guards and continuous real-index exclusion. Late partial failure and corrected explicit retry replace any universal rollback or unchanged-retry inference. Other operations stay on gix; ADR 0004 is unchanged under #50's existing allowance. | Architect authorization in `remediation/git-branches-contract-brief.md`, preserved in the decision receipt: “Architect now authorizes Rust to add exact git2 `=0.21.0` with default features disabled and promote existing workspace tempfile for this checkout-only fallback, then implement the already-approved checkout signature test-first.” Handoff: “StageBdecision (explicitlysupersedesoldindex-beforeHEADproposal)”. | Historical implementation handoff; its nonacceptance boundary is superseded by bounded QA below |
| Git checkout occupancy — handoff received 2026-10-26 at 22:33:42.869 −07:00 | Earlier handoff left other-worktree denial unimplemented → implemented SDK occupancy checks including main/sibling worktrees, excluding own identity and bare main, with an under-target-guard rescan before mutation. Own-current no-op is unchanged. This supersedes the earlier omission, not the existing noncooperating-writer limit. | Product-owner handoff: “Finalactualdecisions:SDKworktreeoccupancyguardincludesmain/siblings,excludesownandbaremain, rescansunderheldtargetguard beforemutating; owncurrentnoopunchanged.” | Historical implementation handoff; pending-QA status superseded below, broader acceptance still incomplete |
| Git checkout acceptance — disposition reconciled 2026-10-27 at 00:55:19.116 −07:00 | Earlier pending/nonindependent wording → bounded independent macOS QA acceptance of implementation and docs at `ad522fe`, based on saved shared core-gate execution and independent probes. Not full #598/#50, Windows/Linux or OS write-fault certification. | `remediation/git598-checkout-commit-disposition.md`; current user instruction: “old598checkout boundedQAaccepted atad522fe”. | Bounded QA accepted at the named commit; not acceptance of subsequent #599 changes |
| Git clone/fetch — handoff received 2026-10-27 at 00:55:19.116 −07:00 | Undelivered transport inventory → implemented explicit clone/fetch and shared zero-I/O Host preflight before clone init/connection construction. Lower-level gix composition avoids destructive cleanup; isolated tracking-only fetch and explicit retained-state recovery preserve local selection. Initial checkout remains gix, not an expanded git2 fallback. | Architect-approved `remediation/git599-contract-brief.md`; user-authorized scope, not direct product-owner approval of implementation details: “Userauthorizedscope, architectownsbrief.” | Historical implementation handoff; initial-clone order, progress and QA disposition superseded below; #600–#602 scope unchanged |
| Git clone/fetch correction — received 2026-10-27 at 02:45:17.203 −07:00 | Earlier clone local branch/HEAD-before-checkout order and forwarded SDK progress → initial checkout/index before local branch/born HEAD, with fixed client progress and actual counters; retain numeric-only SDK checkout progress. Preserve partial state and explicit recovery without weakening #598's HEAD-before-final-index order or missing-index guard. Independent findings supersede the earlier blanket pending-QA description. | Architect-approved `remediation/git599-qa-fix-brief.md`: “initial worktree checkout and its index publication precede publishing the clone's local branch/born HEAD.” Current user: “Existing598checkoutHEAD-beforefinalindexcontract andbornHEADmissingindexguard unchanged.” Scoped execution: `remediation/git599-qa-green/receipt.md`. | Historical: two persistent reds → two greens, supplied progress and six focused selectors passed; original recheck was outstanding and Clippy exited 101. Recheck/lint status superseded below, not full-gate or issue acceptance |
| Git clone/fetch evidence currency — received 2026-10-27 at 07:49:09.067 −07:00 | Pending original recheck/uncommitted candidate → 10/10 original probes and 6/6 configuration regressions PASS, source verifier and bounded macOS implementation/docs recommendation PASS; 21 product paths committed at `ed53dba`. Later focused lint and guarded workspace Clippy/app build/12 app tests passed using the verified offline SDK cache. Earlier failures remain historical; four unguarded fmt/UI successes are not guarded passes. | `remediation/qa-git599/reports/recheck1-final-disposition.md`, `git599-approved-repair-receipt.md`, `git599-commit-gate-handoff.md`, `git599-guarded-gates-receipt.md`; current user: “Correct CURRENT status with minimal factual edits.” | Historical bounded QA acceptance; full workspace tests/UI E2E awaited loopback approval then, superseded below. No full nine-gate, #599/#50, hosted/OS/keychain acceptance or #599 publication; #600 remains unimplemented |
| Git evidence reconciliation — received 2026-10-27 at 16:32:45.141 −07:00 | Preserved the accepted #599 correction; then recorded #598 independent QA/docs/local-gate acceptance at `63753092452443473234ad129945812f97032e34`, tree `0390135031163070d1663a73df29ff062615be70`, and draft PR #614 awaiting native CI. That source had local 734-test evidence, but all three native Crates jobs subsequently FAILED. Earlier loopback/cache-copy permission blockers were resolved. | Earlier user handoff: “publishedPR614DRAFT awaitingnativeCI”; “#599implementationed53acceptedbounded10+6 but finalpropagatedlocalfullgates/newnativeCI NOTyetexecuted”; “Previouslocalhost/cachecopydenialsresolvedbyexplicitexistingapproval”. Later native-failure status supplied in the current handoff. | Historical local acceptance and failed native source; superseded by the repair below, not current pending CI |
| Git repair readiness and clone/fetch continuation — received 2026-10-27 at 18:22:23.801 −07:00 | #598's earlier failed native source → test-only repair `ed093023794b791057bbe308d2c86fd0f173af66`, tree `e9bb58276ce84dce751cad214a3e7b9f6a8eddbf`, all seven LOCAL criteria accepted and required native CI SUCCESS at the parent 18:12:22 receipt; PR #614 READY with actual head/base/body readback complete after 18:19:12. CodeQL NOT_TRIGGERED, not PASS. The user released #599's pause; its original `ed53dba4f90f6e6cd24355738ff19047fde99bd5` 10+6 acceptance remains bounded and final propagated full gates/new CI have not run. | Current user handoff: “PR614READY actualhead/base/bodyreadbackcomplete after18:19:12”; “Userexplicitlyreleased599pauseonfullPASS.”; “freshpropagatedfullgates/newCI NOT YETRUN”. | Historical input-stage reconciliation: its not-run statement records the 18:22 handoff, superseded by the developer evidence below; no new architecture/product decision, transfer of gate results, issue closure or final #599 acceptance |
| Git propagated-tree evidence — handoff received 2026-10-27 at 18:57:53.750 −07:00 | The 18:22 input-stage not-run status is historical. The final-integration developer handoff records twelve passing local gates on tree `69efec6c880e98991d78b5ae6ded520ef01526bb`; keep those results separate from independent QA, exact-head native checks and every successor tree. Restore explicit full-original-criteria caveats without changing the contract. | Current user handoff: “NOWdeveloperall12finaltreegatesPASS794workspace12app134UI104E2E3+3generator and13localhostAPIfixtures”; `remediation/git599-final-integration/developer-handoff.md` and `evidence-summary.json`. | Developer local evidence recorded; no transfer to a changed tree, independent QA signoff, full #598/#599/#50 acceptance, new decision or issue closure implied |
| Git push inventory and fallback — relay received 2026-10-27 at 22:38:54.099 −07:00 | Earlier checkout-only/no-transport and unimplemented-#600/wait-for-#599 inventory → bounded two-method push implementation: gix validation/fresh observation plus git2 upload, HTTPS/vendor features, Host-owned security options and conservative outcomes. Earlier #615 draft/native-wait status is superseded by the named-parent READY receipt. No new ADR, product default or full-original acceptance; prior evidence remains historical. | Architect `git600-contract-brief.md`: “Use existing git2=0.21.0 only for upload, retaining gix=0.90.0 for validation/observation.” Execution release `git600-implementation-brief.md`, October 27 at 19:54:29 PDT; user relay: “Document existingapproveddecisions only, no newADR/default/productdecision”. | Approved contract; implemented bounded code with dated developer proof. Exact successor gates, independent QA/docs and original #598/#599/#600/#50 acceptance remain separate obligations |
