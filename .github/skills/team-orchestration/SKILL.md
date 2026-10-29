---
name: team-orchestration
description: "Dispatch CADE specialists with at most one active agent per writing role (qa-analyst may run as several read-only instances). Use when the architect delegates multi-role work, or whenever issue planning, implementation or fixes are requested."
---

# Team orchestration

Owner: application architect.

## Roles and dispatch

- Role profiles live in `.github/agents/`, one per role. Only the architect has
  the agent tool, so only the architect dispatches. Select `application-architect`
  as the coordinating agent.
- Roles: `application-architect`, `business-analyst`, `project-manager`,
  `designer`, `rust-developer`, `typescript-developer`, `qa-analyst`,
  `document-writer`, `performance-coach`.
- If the main session is not running as the architect, it takes the architect
  role itself, follows `.github/agents/application-architect.agent.md` and this
  skill, and says so to the user. It still dispatches every specialist and never
  does their work.
- Reuse a running or finished agent of a role when the host can resume it,
  instead of launching a second instance of that role.
- Exception: `qa-analyst` only checks and never changes code, documentation or
  the backlog, so several QA instances may run at once, each on a separate
  scope. Every other role changes code, documentation or the backlog and keeps
  one active instance.
- Track milestones in the todo list.
- Every three hours, dispatch `performance-coach` with the review window and
  log locations (see the `team-performance-review` skill). Relay its
  per-agent coaching to running agents, apply its architect coaching to your
  own dispatching, and take its proposals for `AGENTS.md`, instructions,
  settings or guardrails to the user. Every dispatch prompt tells the agent to
  read `.coaching/<role>.md` first.
- Schedule both cadences (hourly progress report, three-hour review) when the
  session starts, and check them again after every resume or compaction. List
  the scheduled prompts. If either is missing, create it, then list again to
  confirm. In Copilot CLI, use `manage_schedule` with `list`, then `create`
  with an `interval`. Confirm the review job fires every 3 hours around the
  clock: its hour field lists 8 hours, or uses a step such as `1-23/3`. Give
  the user a next-review time only from a schedule listing made in that turn.
- Maintain a separate ten-minute blocker reminder throughout delivery and
  verify it after every resume. Report a genuine current blocker immediately,
  then every ten minutes until resolved, including while awaiting approval.
  Lead with **ACTION REQUIRED - WORK BLOCKED** when the user must act, or
  **WORK BLOCKED - FIX IN PROGRESS** when no user action is needed. State the
  affected task, owner, plain-language cause, exact action or approval reply
  needed, what work can continue and the next reminder time. Do not bury a
  request in an hourly table or rely solely on an approval dialog the user
  cannot access. Do not repeatedly invoke an unavailable dialog.
  Record blocker and resolution evidence; reconcile fresh receipts and user
  approvals before repeating an alert. Never infer consent from silence.
  Stop the resolved alert immediately; do not repeat stale blockers or present
  ordinary running checks and queued dependencies as blocked work.
  This reminder supplements, rather than replaces, hourly progress and CI
  monitoring. Stop its schedule only when all scoped delivery is complete or
  the user explicitly pauses the reminders.
- While a plan waits for approval (`exit_plan_mode`), scheduled prompts
  don't fire. Say so when you ask for approval. After approval, list the
  schedules again, and start the next review's window where the last one
  ended, so it covers the pause.

## Steps

1. Confirm the user's goal and which side effects are allowed. The main assistant
   must invoke or resume the application architect for issue delivery, not act as
   an unnamed replacement or dispatch implementation specialists directly.
   - Before asking approval for a push or a deletion, measure its real scope:
     commits that a push would publish (`git log origin/<branch>..<branch>`),
     and bytes and files that a delete would remove (`du`). Put those numbers
     in the question. If the real scope turns out materially larger than what
     the user approved, ask again before acting.
   - Never delete or clean `target/` directories, scratch copies or caches
     that you didn't create in this session without asking.
2. Get dependency-ready issues from `project-manager` (milestone M1 first,
   then by `priority:` label and native blocked-by order) and clarified
   acceptance criteria from `business-analyst`. Unresolved questions block
   only the work they affect. Put product-owner questions to the user. Asking
   blocks the session until the user answers, so:
   - First dispatch every task that doesn't depend on the answer. That
     includes blockers a specialist has already reported with options:
     decide each one, or send it to the role that owns the code, before
     you ask.
   - Ask only questions that block the issue being built now. Log every
     other open point on the issue it affects, and in
     `docs/OPEN-QUESTIONS.md`, and keep building. Don't put it to the user:
     not in a status update, not as "no rush", and not as a check of an
     answer already given. It gets asked when that issue is built. Approval
     asks that `AGENTS.md` requires, such as pushing, stay.
   - Before you ask about a topic the user has already answered, quote
     that answer and its decision record, and use the user's terms. Never
     offer an option the record rules out. The fixed technical decisions in
     `docs/adr/` are not open questions.
   - Ask one decision per question, in plain words: what happens with each
     option and what waits on it, with one concrete example per option.
     Leave out tool internals.
   - If the user hasn't written in the last 30 minutes, don't use a blocking
     question prompt. Ask in a normal message that ends your turn. That
     leaves the session free to handle hand-backs and hourly reports until
     the user returns.
   - When the host declines question prompts automatically, don't use them.
     Copilot CLI autopilot answers `ask_user` at once with "The user is not
     available". Ask in your turn message instead, give the default you will
     apply, and keep that default inside the scope the user set.
   - In each option, name every branch move, reset or deletion it implies. If
     you state a default for no reply, apply it no earlier than the time you
     gave.
3. Agree on contracts and assign write scopes that don't overlap. Shared
   contracts (`cade-core` types, IPC commands, `.cade` format, `policy.yml`,
   `tokens.json`), the root `Cargo.toml`, `Cargo.lock` and
   `app/ui/pnpm-lock.yaml` have one owner at a time and are edited in turn.
   - A new type, wrapper, alias or name for something the code or the docs
     already have is the user's decision, even when a specialist lists it as
     its default. Before a brief, ruling or question uses one, find its
     approval in the issue or an accepted ADR. If there is none, ask the user:
     give the existing type as the first option, and quote the documented
     convention the new one departs from, with file:line.
   - Before a brief asks for a new file, type, crate or constant, `git grep`
     the value or concept and quote the existing definition in the brief.
     Tokens, IPC types, validation and formatting each have one definition,
     so never ask for a second copy "for TS too".
   - At the IPC boundary, write the command name, argument and return types
     and error cases into both the Rust and the TypeScript brief.
4. Keep one registry row per role, including your own, and one row per active
   QA instance:
   `role | agent/run ID | issue | branch | write scope | status | queued work`.
   A role is busy while its agent is running, waiting or idle but resumable. Treat
   unknown status as busy. Rebuild the registry after a session resumes.
   Edit the registry in the same turn as every launch, assignment, hand-back
   and commit, and redo a failed edit at once. Update the row of every agent
   that handed back in that turn, not only the row of the agent you send next.
   - In a turn that handles a user message or a hand-back, make `date
     '+%H:%M:%S'` your first tool call, alone, and wait for its output. Use
     it, labeled "received by", as that event's time in every brief,
     registry line and record request of the turn. Never write a time from
     memory or an estimate; a `~`, an `x`, "about HH:MM" or a `$(…)` in a
     time you send is a defect, because messages to agents don't run the shell.
5. Reserve a free role before launching it. If the role is busy, queue the work or
   reuse that agent. Never launch a second instance of a writing role.
   Before launching an agent for a writing role, read that role's registry
   row in a separate tool call. Start the brief with "Registry row: <role>
   <agent ID> <status>". If that agent isn't stopped, reuse it instead.
   An agent whose run finished can still be reused, so its role stays
   busy. A role is free only when its agent failed, or was stopped because
   it failed or ran out of context. Never retire a reusable agent so you
   can launch a new one. A new `qa-analyst` instance may be launched while
   others run, provided each checks a distinct scope, stays read-only and
   keeps its notes in its own scratch directory.
6. Run different roles in parallel only when their prerequisites are met and their
   write scopes don't overlap. Each assignment includes context paths, issue
   references, the design export when there is one, inputs, allowed files,
   contracts, acceptance criteria and when to stop.
   - Branching (`AGENTS.md`): trunk-based development on `main` with stacked
     PRs. Going forward, every issue and subissue gets its own
     `issue-NN-<scope>` branch. Create the parent's own branch before any
     child; every child must descend from it. List issues in dependency order,
     preferring sequential descendants over conflicting siblings. The user
     merges the stack bottom-up so `main` stays clean and working.
   - Complete already-scoped work; preserve previously approved legacy grouped
     branches and commits. Do not rewrite them to retrofit this prospective
     rule. Maintain a branch map: issue/subissue, parent issue, branch,
     predecessor/base and exact SHAs. Record actual ancestry and merge-conflict
     checks, their tested tree identities and results; names or registration
     alone do not prove a valid, conflict-free stack.
   - Use the `gh stack` command/extension to register one local stack per parent
     and its children. The installed official `github/gh-stack` 0.3.0 `init`
     operation adopts existing branches bottom-to-top with an explicit
     `--base`. Local registration is not publication authorization: no
     `sync`/`submit`, push-capable `link`, push or PR publication is authorized
     by this instruction. Rebase rewrites history and requires approved scope;
     do not invent a dry run. If a lower branch changes, restack descendants
     only within approved scope before continuing.
   - An empty parent anchor at the pre-child base establishes ancestry but
     has no publishable root diff. Report that limitation; do not manufacture
     a diff or claim publication readiness. Registration, ancestry, conflict
     checks and publication readiness are separate evidence obligations.
   - Parallel writers share an issue's branch only with disjoint file scopes;
     distinct issues/subissues do not share a new branch. Shared files are
     edited by one role at a time and committed before the next role touches
     them. Opening PRs requires pushing, which needs the user's approval.
   - A tree under QA or design review has no active writer in the files under
     review. Sequence the work, or have QA review from a scratch copy taken at
     a named commit, rather than creating another branch.
   - Name the tree path and the commit SHA in every prompt that cites
     repository state, including project-manager and business-analyst prompts.
   - Create only the issue/subissue branches in the issue plan you've shown
     the user. If a new issue/subissue appears, add it to the plan and name it
     in your next status update before you create the branch. Never create a
     branch, worktree or merge target as a fix for a process problem without
     asking.
   - Before every commit, run the applicable build commands against the exact
     candidate tree and require success. Confirm the staged content matches
     the validated tree; rerun affected checks after further edits. Missing,
     blocked, skipped or stale evidence is not a successful build. Do not
     commit a known-broken tree or defer its repair to a later layer.
     If a local build or hosted build check fails, stop unrelated feature
     work and publication; repair and verify the failure before resuming.
     Post-push exact-head platform checks below remain mandatory.
   - Maintain automatic CI monitoring for every commit across all delivery
     work, not just selected PRs. Verify the monitor is active at startup
     and after resuming. Register each new commit's SHA, branch, PR when
     available, applicable checks and build evidence in the delivery registry.
     Check published commits every five minutes until results are resolved;
     retain their evidence after branch heads move. A passing descendant
     does not prove an earlier commit passed. For unpublished commits, retain
     local build evidence and explicitly mark hosted CI awaiting publication.
     Immediately route failures to the architect for priority repair. Keep
     monitoring new commits until all scoped delivery is complete or the
     user pauses it, rather than stopping when an individual PR passes.
   - When building or restacking a level, run the full gate on that level's
     exact SHA before the next level starts:
     - `cargo fmt --all --check`;
     - `cargo clippy --workspace --exclude cade-app --all-targets -- -D warnings`;
     - `cargo test --workspace --exclude cade-app`, the whole workspace,
       never a hand-picked `-p` list: a change to `cade-core` breaks every
       crate that uses it;
     - `pnpm --dir app/ui build`, `typecheck`, `test` and `test:e2e`, plus
       `cargo build -p cade-app` and `cargo test -p cade-app`, for any level
       that touches `app/` or a crate `cade-app` depends on.

     Save each result as a log named `<level>-<sha>` that lists the test
     names. Put each fix at the lowest level whose code it changes. Explain
     any change beyond the original pick in that commit's message.
   - Publish PRs as drafts until independent QA, documentation and all
     applicable required checks pass on the exact current PR head. For
     cross-platform behavior, verify the relevant Windows, macOS and Linux
     CI results; local green checks do not establish cross-platform readiness.
     A failed, pending, missing or skipped relevant check is not a pass.
     Keep affected PRs in draft while any required evidence is unavailable.
     After a push or stack propagation, recheck each affected PR's current
     head rather than citing results from an older SHA. Do not weaken tests,
     edit protected workflows or bypass checks to make a PR ready.
   - For a reported regression, trace the introduction and any prior fix,
     reproduce the failure before changing production, and retain a focused
     regression test. Exercise the failing platform boundary locally when
     practical and rerun the originally failing platform check. Generated
     artifact checks must preserve the specified canonical output; do not
     normalize away a discrepancy merely to silence a drift failure.
   - Batch replays. Before replaying, collect every pending fix for lower
     levels, including fixes for known flaky required tests, so that one
     replay covers them all. Before moving branch heads with `git branch -f`,
     save the old-to-new SHA map to a file in a scratch directory outside the
     repository.
   - When a fix lands at a low level, have QA review it from an export first.
     Replay the levels above and start stack-wide heavy reruns only after
     that QA approves.
   - Cargo builds are heavy. Give each parallel lane its own
     `CARGO_TARGET_DIR` in a scratch directory, never the repo's `target/`
     while another lane builds, and run at most two full workspace builds at
     a time. Light lanes (git, `cargo fmt`, typecheck) can run alongside.
   - When task B needs task A's code, dispatch B only after A is committed,
     and give B's role other ready work in the meantime.
   - Disjoint write scopes aren't enough when one role's build compiles
     another role's files. If B builds against crates A is editing, dispatch
     B after A hands back, or point B's build at a frozen copy, and say
     which in both briefs.
   - After a permission denial, don't retry that action yourself or through
     another role unless the user directs it. Report it and ask for the permission.
7. Route questions between roles yourself: requirements to `business-analyst`,
   labels, dependencies and status to `project-manager`, decisions to
   `document-writer`.
   - `business-analyst` and `project-manager` can't write files, so their
     reports exist only in the session log. In the turn each one arrives,
     save it word for word under the session's `files/` directory and name
     the path in the registry. Extract it, never retype or summarize it: in
     Copilot CLI, `sed -n '<line>{p;q;}' events.jsonl | jq -r .data.content`.
     Make that save your next tool call after the report arrives, before
     you read anything else.
8. Treat developer completion as ready for QA. Dispatch `qa-analyst` explicitly
   with acceptance criteria, affected existing behavior and consumers, regression
   risks, changed files and developer test evidence. QA independently checks
   acceptance and regressions; neither architect review nor developer tests
   substitute for this handoff.
9. Route blocking QA findings to the responsible developer, then send fixes and
   red/green evidence back to QA. Require rechecking the original failures and
   affected regressions. Repeat without duplicating role owners.
10. Integrate results and resolve interface mismatches. Declare completion only
    with independent QA's completion recommendation, passing required checks and
    completed dependencies, decision records and documentation. Report unexecuted
    or blocked checks explicitly. Ask before issue closure.
    - When a role is freed, dispatch its next queued task. Before you end a
      turn, read the registry: dispatch every idle role that has ready work,
      or name in your message what it waits on.
    - In the same turn as each commit that carries a ruling or default, yours
      or the user's, send `document-writer` the list of them; if it can't be
      reached, launch a fresh instance.
    - Check the branch in the same command as every commit:
      `test "$(git branch --show-current)" = <the registry's branch> &&
      git commit …`. Use `git status -sb`, which prints the branch, in every
      status check before staging.
    - If a QA verdict is conditional ("ready to commit provided X"), don't
      commit, merge or report approval yet. Send that QA instance the
      evidence for X (for a commit-message condition, the exact draft message
      and the staged path list), act only after it replies that X is met, and
      quote X and that reply in your report to the user.
11. Update todos on evidence-backed milestones, not merely when an agent starts
    or stops. Publish progress to the user at handoffs, blockers and the cadence
    in `AGENTS.md`: active agent names and roles, actual states, completed
    milestones, commands and outcomes, blockers and next handoffs. Require
    specialists to report the same information to the architect at the same
    cadence, at meaningful boundaries and in every report.
    - Every hourly update lists each outstanding task in the current scope,
      including active, queued and blocked work, with its owner, status and
      estimated remaining-duration range. Include the estimate's evidence,
      assumptions and uncertainty; do not substitute one overall ETA for
      per-task estimates.
    - Separate implementation, independent QA and publication time from waits
      for dependencies, approvals or external validation. If an estimate is
      not supportable, report "unknown" with the specific blocker or missing
      evidence and the next estimation checkpoint. Never invent a duration.
    - Build a hand-off estimate from the list of remaining steps, each with its
      duration measured in this session, and show that list. Re-derive it
      after every milestone and hourly update instead of carrying the previous
      figure forward; explain material changes. Distinguish remaining work time
      from a completion ETA, accounting for dependency waits and parallel work.
    - Each time you tell the user you've dispatched work, give its expected
      hand-back time when supported by durations measured in this session;
      otherwise use the explicit unknown-estimate treatment above.
    - In each hourly report, run `gh issue list -R a11smiles/cade --state open
      --limit 10 --json number,title,author,createdAt` and name every issue
      created since the previous report (including feedback filed from the
      app), with the role you sent it to.
    - For any background job expected to run over 10 minutes, require the
      specialist to name a progress file and the total count before it
      starts. When the user asks for the status of a running agent, report
      from that file with one read-only count, for example "about 100 of 200
      runs, 0 failures, done at about HH:MM". Don't say only "no results yet".
12. On failure, confirm the failed role and any children have stopped. Preserve
    files, evidence and completed milestones; replace only the failed role and
    resume its unfinished scope rather than restarting implementation.
    - After plan approval, don't send file-writing work to an agent that was
      launched while plan mode was on: it can keep refusing writes. Mark it
      stopped in the registry and launch a fresh instance of the role with
      its saved diff or notes.

If subagents aren't available, don't do specialist work yourself; return an ordered
dispatch plan (roles, assignments and write scopes) to the main session/host for dispatch.
Do not label self-verification as independent QA or declare the independent-QA completion gate satisfied.

## Branch-rule change history

- October 26, 2026 — The product owner's prospective delivery instruction
  supersedes the earlier one-branch-per-planned-task rule (including multiple
  task branches for one issue) and its blanket sibling prohibition:
  “complete the work you have scoped. however, going forward, all issues and
  subissues receive their own branch. ... use the gh stack command/extension
  to commit all subissues with their parent issue as a single stack ...
  subissues, at a minimum, will need to be a branch of the parent issue.”
  The current rule is one branch per issue/subissue, parent before children,
  preferably sequential descendants, and one local parent/children stack.
  Previously approved grouped branches/commits remain valid legacy history;
  no retrospective rewrite is authorized. This is delivery guidance for the
  #50/#598 handoff, not new product scope or independent backlog completion.
  Existing future `Fixes`/`Closes` eligibility, independent QA and permission
  gates remain unchanged.
