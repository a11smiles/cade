# Contributing to CADE

Thanks for helping. CADE is a personal open-source project maintained by Joshua Davis under Apache-2.0.

## Find something to work on

The plan lives in [GitHub issues](https://github.com/a11smiles/cade/issues), with backlog ids (for example `[CORE-004]`), acceptance criteria and dependencies. Start with the issues labeled `milestone:M1`. Comment on the issue before starting anything large.

## Build

```sh
# Rust crates (no system webview needed)
cargo test --workspace --exclude cade-app

# UI
pnpm --dir app/ui install --frozen-lockfile
pnpm --dir app/ui build
pnpm --dir app/ui typecheck
pnpm --dir app/ui test
pnpm --dir app/ui exec playwright install --with-deps chromium
pnpm --dir app/ui test:e2e

# Desktop app (needs the platform webview: WebView2 on Windows, WKWebView on macOS,
# webkit2gtk-4.1 on Linux)
cargo build -p cade-app
cargo test -p cade-app
```

These commands have recorded CI or remediation execution evidence. That evidence
is tied to its tested commit and platform, not a claim that every checkout or
native environment passes. Source builds do not create an available installer.

## Rules of the road

- Participation is governed by the [Code of Conduct](CODE_OF_CONDUCT.md).
- One backlog item per pull request where possible. Put the id in the PR title.
- Every future code commit must reference an actual backlog issue number in its commit message. Keep code commits and PRs scoped to identified backlog work; do not bundle unrelated work.
- Link existing code history through issue evidence comments with commit SHA links; never rewrite history merely to add attribution. A commit alone does not establish issue completion.
- Keep formatting, lint, crate, UI and applicable desktop checks green. The recorded workspace checks are `cargo fmt --all --check`, `cargo clippy --workspace --exclude cade-app --all-targets -- -D warnings`, and the test commands above.
- UI values come from design tokens only. Edit `design/system/tokens.json`, never `tokens.css`.
- Check dark and light themes.
- Don't commit Microsoft's Azure icon files (see `assets/icons/azure/README.md`).
- No telemetry, no network calls outside the ones a backlog item names.

## Requirements and acceptance evidence

Before implementation, define explicit contracts: actors, preconditions, inputs,
observable results, state transitions, and permitted or prohibited side effects.
Write testable Given/When/Then examples for success and applicable boundaries,
denial, failure, recovery, cross-platform behavior and integration consumers.
Separate confirmed requirements, approved interim behavior, open decisions and
excluded scope. Do not silently weaken original acceptance criteria or treat an
assumption as approval.

Work with more than seven acceptance criteria or too large for a one–five-day
delivery must be decomposed before implementation into bounded native GitHub
subissues. Reuse existing issues where appropriate. Each child needs its own
contract, estimate, evidence requirements and actual native blocked-by
relationships for its prerequisites; a checklist of links is not dependency
wiring. Preserve the parent and all of its obligations. Completing children does
not automatically complete the parent.

For each behavior change, first demonstrate the expected failing assertion, then
the minimal fix and a passing run. An unexpected initial pass does not reproduce
a bug. Preserve existing tests and assertions; never skip, delete or weaken them
to get green results. Documentation, configuration and styling changes are exempt
from behavioral red-first testing; do not manufacture test evidence for them.

Acceptance evidence identifies the exact command and test, commit SHA, working
tree changes if any, platform, expected and actual results, exit status, and raw
failing-then-passing logs. Include proof at the real affected consumer boundary:
for example, actual Rust feedback drafts versus submitted snapshots, generated
output checked by its target parser/compiler, or database execution when the
original criterion requires it. Mocks, generated strings and aggregate green
suites do not substitute for native, hosted-service or database evidence.

Independent QA must assess the original criteria and regressions, with current
documentation before completion. Attribute shared gates to their executor;
do not relabel them as reviewer-run checks. Report blocked and unexecuted checks
per issue, without turning a bounded fix approval into whole-issue, cross-platform
or release approval. A commit or closed issue alone proves none of these.

## Maintaining feedback contracts

The three `.github/ISSUE_TEMPLATE/*.yml` files own form metadata; Rust in
`cade-feedback` owns payload types, validation and redaction. Consume the generated
`app/ui/src/ipc/bindings.ts` declarations (ts-rs) and
`app/ui/src/feedback/forms.generated.ts`, not handwritten copies.
`types.ts` is a display facade. Keep generation/staleness checks read-only during
ordinary gates.

Changes must account for Rust callers and migration fixtures, preview/submit IPC,
the dialog and its tests, generated artifacts, the e2e IPC mock, hosted YAML and
label inventory, and `cade-github`'s issue-draft consumer. Preserve required fields,
legacy payload migration, opt-out/unavailable-data behavior, and the approved
copy. The named `version` → `app_version` correction, labels and copy are resolved,
not new clarification blockers. See the
[feedback contract](docs/ARCHITECTURE.md#feedback-contract) for the current decision.
User-owned CI wiring for #309/#310 is separate from local schema validation and
is not authorized by a form change.

## Feedback without code

Choose the issue form that best fits your feedback:

- **Bug:** Something is broken or wrong.
- **Feature request:** A capability or change CADE should add.
- **Comment or question:** Praise, questions, or general feedback. Comment submissions are triaged weekly and may be converted to Discussions.

You can also use Help › Send feedback inside CADE.
