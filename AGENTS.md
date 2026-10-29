# AGENTS.md

Keep this file under 500 words; put role details in
[agents](.github/agents/), [skills](.github/skills/) and
[instructions](.github/instructions/).

## Operations

- Route issue planning, implementation and fixes through
  [`application-architect`](.github/agents/application-architect.agent.md);
  the main assistant must not substitute itself. Only the architect dispatches
  specialists, selected automatically by responsibility.
- Follow [team-orchestration](.github/skills/team-orchestration/SKILL.md) for role
  ownership, evidence-backed todos, handoffs and failure recovery.
- Developer completion means ready for independent `qa-analyst` acceptance and
  regression checks. Route fixes through the architect back to QA. Completion
  requires passing checks, QA approval and completed documentation; report
  blocked/unexecuted checks honestly.
- Architect and specialists report progress automatically every hour:
  agent names/roles/states, milestones, results, blockers and next handoffs.
- Ask before closing issues, pushing, releasing or destructive actions.
- Use American English everywhere: communication, docs, code, comments, copy.
- Every agent reads `.coaching/<its role>.md` (if present) before starting
  and follows it. Every three hours the architect dispatches
  [`performance-coach`](.github/agents/performance-coach.agent.md) and relays its coaching.

## Context and ownership

- Requirements: `a11smiles/cade` issues, titled `[EPIC-NNN]`. Decisions:
  `docs/adr/`. Never implement unresolved `docs/OPEN-QUESTIONS.md` entries.
- `business-analyst` owns issue content; `project-manager` owns labels,
  dependencies, milestones, project status and closing; `document-writer` owns `docs/`.
- User-owned CI/CD, signing and release configuration, including
  `.github/workflows/`, must not be changed.
- `crates/`: Rust libraries. `app/src-tauri/`: Tauri 2 shell (`cade-app`).
  `app/ui/`: React + TypeScript chrome. `design/system/tokens.json`: only
  token source. `design/exports/`: never edited.
- The stack in `docs/adr/` 0001–0007 is fixed; don't re-decide it.
- Use official SDKs, never handwritten service clients.
- Scaffold only when requested. No scripted search-and-replace; use the
  edit tool.

## Integrity and security

- No telemetry or network call an issue doesn't name; the webview never
  reaches the network. Agent edits are staged change sets the user applies.
- Every policy decision goes through `cade-policy`.
- Tokens and BYOK keys live only in the OS keychain. Never log secrets, prompts
  or user content. Feedback is redacted and previewed before sending.
- Never commit secrets, real user projects or Microsoft's Azure icon files.

## Testing

- Every behavior change: write/run a one-behavior test; require the expected
  failure first (bugs must reproduce), then minimal production code and a passing
  run; refactor with SOLID/DRY while green. Unexpected initial passes prove
  neither a new behavior nor a fix.
- No duplicate tests. Split functions needing more than three tests.
- Never skip, `#[ignore]`, delete or weaken tests/assertions to pass.
- Report each new test's failing and passing output; no behavioral completion
  without red-first evidence. Config, styling and docs are exempt.

## Commands and delivery

- `cargo fmt --all --check`
- `cargo clippy --workspace --exclude cade-app --all-targets -- -D warnings`
- `cargo test --workspace --exclude cade-app`
- `pnpm --dir app/ui install|build|typecheck|test` (Vitest);
  `pnpm --dir app/ui test:e2e` (Playwright). Use pnpm, never npm/yarn.
- `cargo test -p cade-app` (IPC); `cargo run -p cade-app`.

Trunk-based `main`, stacked PRs: one branch per issue/subissue.
Parent first; children descend, preferably sequentially. One local `gh stack`
per parent/children (team-orchestration). Preserve approved grouped history.
No agent attribution. PR titles: imperative, `[EPIC-NNN]`-prefixed except bug fixes.
Check UI in both themes.
