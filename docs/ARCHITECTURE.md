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
│     ├─ cade-git     gitoxide: commit per save / per Copilot run         │
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
- **One identity.** GitHub device-flow sign-in gives the token used for Copilot, shipping, review pages and feedback issues.
- **Policy everywhere.** `cade-policy` decides before Copilot runs a tool, before BYOK is used, and before anything ships.
- **Copilot proposes, the user applies.** Agent output is a staged change set. Applying it is one commit with run trailers.

See `docs/adr/` for the decisions behind this; the accepted
[ADRs](adr/) remain unchanged.

## Delivered capabilities and boundaries

The [README crate table](../README.md#implemented-starting-points) inventories
existing code. The following sections distinguish implemented remediation from
bounded approval and pending verification; none establishes release readiness.

### Git implementation boundary

**Status:** local open/init/status foundation (#596 / GH-024), explicit-tree
commits (#597 / GH-025), and branch creation/merge-base (#598 / GH-026 Stage A)
implemented. Stage A independent QA is pending; checkout remains gated and
undelivered. This is not completion of #598 or #50 / GH-005 under
[ADR 0004](adr/0004-git-is-history.md), the unchanged Git-history authority.

The approved local contract is implemented in
[`cade-git`](../crates/cade-git/src/lib.rs). Existing `ProjectRepo` and
`GitError` remain the API, with
`ProjectRepo::status(&self) -> Result<Vec<gix::status::Item>, GitError>`.
Consumers are crate callers/tests, not app IPC; `cade-app` does not depend on
this crate. There is no new shared status wrapper, serialized format, `.cade`
or policy schema, dependency, or git2 fallback.

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
  than false-clean results or recursive traversal. Production uses gix without
  subprocess Git, shell or network operations.

HEAD inspection and Copilot commit-message formatting also remain available;
the commit operation below stores supplied messages rather than generating
attribution. Remaining work includes #598 checkout and full acceptance, #599
authenticated clone/fetch, #600 push, #601 configured LFS, and #602 typed
cancellable job/IPC integration. Commit-per-save/run and full Git/app integration
remain undelivered. These local mechanics preserve the parent obligation; they
do not authorize transport, LFS or IPC contracts for later layers.

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

Stage A uses the pinned gix 0.90.0 with unchanged dependencies. No checkout
stub, git2 fallback, wrapper, error variant or app IPC was added. #50 permits a
narrow future feature-specific fallback for the missing existing-worktree
checkout capability; that permission is not dependency adoption or delivery.
Stage B checkout remains gated on SDK-owned private-index/continuous real-index
lock and guarded-reference safety proof, followed by production authorization.
ADR 0004 remains accepted and unchanged; this is not permission to replace
gitoxide generally.

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
parented commits, branch creation and explicit-commit ancestry.
The [commit probes](../crates/cade-git/tests/repo/commit.rs)
inspect selected trees, parents, stored bytes and trailers, with separate
validation, preservation, lock, stale-tip and retry cases.
The [branch probes](../crates/cade-git/tests/repo/branches.rs) cover strict
creation, supplied actors, competing creators and corrected retries;
[ancestry probes](../crates/cade-git/tests/repo/merge_base.rs) cover unique,
multiple-best, unrelated and invalid/incomplete histories with preservation.
Git CLI use in these tests builds fixtures or independently inspects results;
it is not the production implementation. Stage A independent QA remains
pending; developer red/green results and focused checks are not acceptance.
The two Stage A Linux-only raw-byte fixtures remain unexecuted, not passed.
Local repository evidence does not establish actual save/run integration,
hosted GitHub/GHES authentication, checkout/transport/LFS/job delivery,
all-platform behavior or release readiness. In particular, nonroot macOS
permission fixtures do not certify Linux permissions or Windows ACL behavior.
Independent QA and exact-commit full-workspace gate evidence must be assessed
separately; earlier #597 gates do not certify Stage A changes. Moving gate
results and test counts are not maintained in this architecture record.

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
| Git branches/ancestry — handoff received 2026-10-26 at 20:20:14.247 −07:00 | Earlier inventory excluded all #598 operations → implemented Stage A `create_branch`/`merge_base` under `remediation/git-branches-contract-brief.md`, checked against source. Checkout remains gated; #50's narrow future fallback permission is recorded without dependency adoption or ADR modification. Also correct #597's missing-only preflight description to its delivered full observed-reference preflight, retaining race/rollback limits. | Current handoff: “Document only actual StageA capabilities and remaining checkout/transport/LFS/jobs/platform gaps.” “Accepted ADR0004 unchanged; #50 permits a narrow future fallback but it is not delivered.” | Implemented Stage A; independent QA pending; neither #598 nor #50 complete |
