# Copilot configuration

Agent configuration for CADE. The system's structure is in
[ARCHITECTURE](../docs/ARCHITECTURE.md). Architecture decisions are living
records, one per topic, in [`docs/adr/`](../docs/adr/) (0001 to 0007); each has a
current decision and, once it changes, a dated change history. Requirements are
the `[EPIC-NNN]` issues in `a11smiles/cade`, tracked in the "CADE backlog"
project.

| Path | Purpose |
| --- | --- |
| `../AGENTS.md` | Always-loaded rules: ownership, invariants, security, testing, commands (keep under 500 words) |
| `copilot-instructions.md` | One-line pointer to `AGENTS.md`; add nothing else here |
| `../docs/OPEN-QUESTIONS.md` | Unresolved requirement conflicts; affected work waits on them |
| `instructions/` | Coding rules for Rust, TypeScript and CSS/tokens, applied by file path |
| `agents/` | Nine role profiles; only `application-architect` has the agent tool |
| `../.coaching/` | Per-role coaching notes; every agent reads its own file first, and `performance-coach` updates them every three hours |
| `ISSUE_TEMPLATE/` | Bug, feature request and comment forms; the in-app feedback dialog mirrors them |
| `labels.yml`, `workflows/labels.yml` | Feedback labels, synced on push to `main` |
| `workflows/ci.yml` | fmt, clippy and tests on macOS, Windows and Linux; UI and app builds |
| `workflows/agent-instructions.yml` | Pull request check: `AGENTS.md` under 500 words, Copilot pointer intact |
| `skills/` | Procedures: orchestration, clarification, backlog, scaffolding, design fidelity, testing, decisions, performance review |

## Roles

| Agent | Owns |
| --- | --- |
| `application-architect` | Coordination, dispatch, architecture, cross-crate and IPC contracts |
| `business-analyst` | Issue content and acceptance criteria |
| `project-manager` | Labels, dependencies, milestones, project status, closing |
| `designer` | `design/system/` tokens and component specifications |
| `rust-developer` | `crates/`, `app/src-tauri/`, the Cargo workspace |
| `typescript-developer` | `app/ui/` |
| `qa-analyst` | Independent acceptance and regression verification (read-only; several may run) |
| `document-writer` | `docs/`, decision records, contributor docs |
| `performance-coach` | `.coaching/`, agent and skill refinements |

## Using it

Open the repository root in your Copilot host, trust its configuration, and
select `application-architect` to coordinate work. Agent and skill discovery
depend on the host. GitHub-hosted agents only see these files after they are
committed and pushed. Requirements belong in GitHub issues; decisions belong in
`docs/adr/`. Scaffolding still requires an explicit request.

## Limitations

The one-instance-per-role rule is an instruction-level policy, not a runtime
lock. Use one coordinating session. Running agents across several sessions or
machines safely would need a shared lock or queue.

Agents can only run the platforms they're on. Windows and Linux behavior is
verified by CI or a person on that platform, and QA reports which platforms it
actually ran.
