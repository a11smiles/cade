---
name: architecture-scaffolding
description: "Propose and scaffold new CADE structure: a crate, a Tauri command surface, an app/ui screen area, or a test harness. Use only when scaffolding is requested."
---

# Architecture and scaffolding

Owner: application architect. Having this skill doesn't mean you're authorized to
start a build. The workspace scaffold already exists (`FOUND-001`); this skill is
for adding to it.

1. Take stock of what's in the repository: the root `Cargo.toml` (members,
   `[workspace.dependencies]`, lints), `rust-toolchain.toml`,
   `docs/ARCHITECTURE.md`, `docs/adr/`, `app/src-tauri/tauri.conf.json` and
   `capabilities/`, and `app/ui/package.json`.
2. Follow the fixed technical decisions in `docs/adr/`. Propose options only
   for genuinely open choices and get approval before implementing them.
3. Keep the dependency direction: `app/ui` → IPC → `app/src-tauri` →
   library crates → `cade-core`. `cade-core` depends on no CADE crate; no
   library crate depends on `cade-app`; importers, exporters and generators
   depend on the model, never the reverse. Business logic depends only on
   traits it defines. Leave room for the next variant without building
   speculative abstractions.
4. Get the decisions needed to proceed and have the document writer record them.
5. List CI/CD, signing and platform prerequisites (webview libraries, GPU,
   code-signing identities) for the user, who owns them.
6. Agree on contracts before parallel work: `cade-core` types, error model,
   IPC command signatures, the `.cade` format, `policy.yml` keys and the
   Copilot tool API.
7. Scaffold with the ecosystem's own tools and the pinned configuration:
   - A new crate: `crates/cade-<name>/` with `Cargo.toml` inheriting every
     `workspace` field like the existing crates, a `description`, a `//!`
     summary in `lib.rs` naming its backlog epic, a one-line README, and an
     entry in the root `[workspace.dependencies]`. Add it to the crate tables
     in `README.md` and `docs/ARCHITECTURE.md` through the document writer.
   - A new IPC surface: typed commands in `app/src-tauri/src/`, the narrowest
     capability permission, and a matching wrapper in `app/ui/src/ipc.ts`.
   - A new UI area: a folder under `app/ui/src/screens/` using the shared
     components and tokens, working outside Tauri.
   - A new test runner or harness: propose it first; never add one silently.
     `app/ui` already uses Vitest with Testing Library and jsdom.

   Extend existing configuration rather than replacing it, and don't relax
   lints, `strict` TypeScript or capability restrictions without a decision
   record. Tell the user any new check needs to run in CI so a failing or
   skipped test blocks the merge.
8. Verify the smallest working vertical slice with the full gate in
   `AGENTS.md`. Record any new build, lint, test or run command under
   "Commands and delivery" in `AGENTS.md` (the user approves `AGENTS.md`
   edits) and in `CONTRIBUTING.md`.

Output: approved decisions, scaffold boundaries, contracts, verified commands and
remaining prerequisites. Make no claims of production readiness.
