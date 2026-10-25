# Backlog conventions

Every backlog item is one Markdown file. Nothing else goes in an item file.

## Path

`backlog/<NN>-<epic-slug>/<EPIC>-<NNN>-<kebab-slug>.md`

Example: `backlog/03-core/CORE-004-loro-document-schema.md`

## File format

```markdown
---
id: CORE-004
title: Define the Loro document schema for a .cade project
tags: [core, crdt, feature]
epic: CORE
priority: P0
depends_on: [CORE-001, CORE-003]
design: design/exports/v0001-2026-10-25/Main.dc.html
---

## Description

What to build and why, in 2–6 sentences. Name the crate(s) it lives in.

## Acceptance criteria

- [ ] Observable, testable outcome
- [ ] ...

## Notes

Optional: edge cases, references, open questions.
```

Rules:

- `id` is `<EPIC>-<NNN>`, zero-padded, unique, never reused.
- `title` is imperative ("Render text with Parley"), under 80 characters.
- `tags`: always include the epic tag (lowercase) and exactly one type tag: `feature`, `chore`, `spike`, `bug`, `docs`, `test`. Add surface tags as needed: `ui`, `rust`, `ts`, `windows`, `macos`, `linux`, `security`, `a11y`, `perf`, `policy`.
- `priority`: `P0` (needed for any usable build), `P1` (needed for 1.0), `P2` (after 1.0).
- `depends_on`: ids only; `[]` if none.
- `design`: path to the exported screen this implements, or omit.
- Acceptance criteria must be verifiable by a test or a demo step. 3–7 per item.
- Size items so one person can finish one in 1–5 days. Split anything bigger.

## Epics

| NN | Epic | Prefix | Scope |
|----|------|--------|-------|
| 01 | foundation | FOUND | Cargo workspace, Tauri 2 shell, CI matrix, packaging, signing, auto-update, logging |
| 02 | design-system | DS | CADE's own UI tokens (dark/light/system), components, icons, fonts |
| 03 | core | CORE | Document model on Loro CRDT, .cade format, git persistence, undo, selection, components/variants, tokens binding |
| 04 | canvas | CANVAS | wgpu + Vello renderer, Parley text, tools, transforms, snapping, auto layout (Taffy), constraints, vector editing, exports |
| 05 | copilot | COPILOT | Copilot SDK (Rust), BYOK providers, threads, context, agent tool API, staged change sets, permissions, metering, MCP |
| 06 | app | APP | App output: Design/Prototype/Code modes, React codegen, two-way sync, preview sandbox, flows, LAN preview |
| 07 | image | IMAGE | Image output: synced sizes, shapes, effects, brand lock, raster/vector export |
| 08 | deck | DECK | Deck output: slides, masters, charts, notes, present mode, .pptx import/export |
| 09 | document | DOC | Document output: pages, pagination, text styles, blocks, print check, PDF/.docx/.md |
| 10 | motion | MOTION | Timeline, keyframes, easing tokens, reduced motion, MP4/GIF/Lottie/CSS export |
| 11 | diagram | DIAGRAM | Shapes, connectors/routing/glue, swimlanes, stencils, Visio import/export, Mermaid |
| 12 | architecture | ARCH | Azure only: resource catalog, official icon pack loader, containment, Bicep + Terraform azurerm, lint, what-if |
| 13 | data-model | DATA | Entities, relationships, enums, classifications, DDL for PostgreSQL/SQL Server/MySQL/Oracle, migrations |
| 14 | system | SYS | Linked Data → App → Architecture graph, bindings, impact, deploy plan, Actions workflow generation |
| 15 | github | GH | Auth (device flow), identity, git ops, Ship to PR, review pages, PR comment sync, GHES/GHE.com |
| 16 | policy | POLICY | policy.yml schema, distribution, enforcement, admin console, budgets, audit |
| 17 | import | IMPORT | Spark repos, Figma REST API, PowerPoint, Visio routing, fidelity reports |
| 18 | history | HIST | Version timeline, compare, restore, branch, review links |
| 19 | onboarding-settings | ONB | First run, settings, appearance, shortcuts, Azure sign-in, keychain |
| 20 | feedback | FEED | GitHub issue types (bug/feature/comment), templates, in-app submission with the user's identity |
| 21 | quality | QA | Tests, accessibility, performance budgets, security/threat model, fuzzing |
| 22 | release-oss | REL | License, governance docs, docs site, distribution channels, naming/trademark |

## Fixed technical decisions (do not re-decide in items)

- Shell: Tauri 2; app chrome is React + TypeScript (Vite) in the webview.
- Canvas: native wgpu surface rendered with Vello, composited under a transparent webview. Text via Parley. Layout via Taffy.
- Document model: Loro CRDT (MovableTree for the layer tree). Saved as `.cade` (Loro snapshot + updates) inside a git repo, with a JSON export for diffs.
- Copilot: `github-copilot-sdk` Rust crate (bundles the Copilot CLI) and a `ModelProvider` trait with BYOK providers (Anthropic, OpenAI, Azure OpenAI, OpenAI-compatible). Both supported in v1.
- Usage is metered in tokens, not premium requests.
- Platforms: macOS, Windows, Linux for every release.
- All output types ship in 1.0: App, Image, Deck, Document, Motion, Diagram, Architecture (Azure only), Data model.
- Figma import uses the Figma REST API with a user-supplied token.
- Policy: `cade/policy.yml` in the org's `.github` repo, plus Copilot policies where the SDK exposes them.
- Ship/deploy only through pull requests and GitHub Actions with OIDC.
- Feedback: bug / feature request / comment submitted as GitHub issues using the signed-in user's identity.
- Azure icons: official Microsoft Azure Architecture Icons SVGs dropped in `assets/icons/azure/` (not committed if license disallows redistribution; loader reads that folder).
- License: Apache-2.0. Personal OSS project owned by Joshua Davis.

## Crates

`cade-core`, `cade-crdt`, `cade-render`, `cade-text`, `cade-layout`, `cade-copilot`, `cade-policy`, `cade-git`, `cade-github`, `cade-io-visio`, `cade-io-figma`, `cade-io-pptx`, `cade-io-spark`, `cade-io-export` (PNG/SVG/PDF/MP4/GIF/Lottie), `cade-codegen-react`, `cade-iac` (Bicep/Terraform), `cade-ddl`, `cade-system`, `cade-feedback`, app in `app/src-tauri` (crate `cade-app`) and `app/ui`.
