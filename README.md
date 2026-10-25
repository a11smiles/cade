# CADE — Copilot ADE

CADE is a local, cross-platform design workbench driven by GitHub Copilot. It's meant to replace Figma, Visio and Spark for enterprise teams. You can design apps, images, decks, documents, motion, diagrams, Azure architecture and data models, then ship them as pull requests. Everything runs on your machine with your Copilot seat or your own model key, under your organization's policy.

> **Status: pre-alpha scaffold.** The workspace builds and its tests pass. The full plan is the [issue backlog](https://github.com/a11smiles/cade/issues): 535 items, starting with milestone M1.

## What's here

| Path | What |
|---|---|
| `crates/` | 19 Rust crates: document model, CRDT, renderer, Copilot, policy, GitHub, importers, generators |
| `app/src-tauri/` | Tauri 2 desktop shell (`cade-app`) and its IPC commands |
| `app/ui/` | React + TypeScript UI: Start screen, theming and the feedback dialog, so far |
| `design/system/` | CADE's own design tokens (dark and light) and brand book. Source of `tokens.css` |
| `design/exports/` | Versioned UI design exports. Never overwritten; see its README |
| `assets/icons/azure/` | Drop Microsoft's Azure Architecture Icons here (not committed) |
| `docs/` | Architecture overview and decision records |

## Build

```sh
cargo test                        # all crates; no webview needed
pnpm --dir app/ui install
pnpm --dir app/ui build           # regenerates tokens.css, type-checks, bundles
pnpm --dir app/ui test            # UI tests (Vitest + Testing Library, jsdom)
cargo run -p cade-app             # desktop app (needs the platform webview)
```

Linux needs `libwebkit2gtk-4.1-dev`, `librsvg2-dev` and `libsecret-1-dev`.

## Starting points already in code

| Crate | Working today | Backlog |
|---|---|---|
| `cade-core` | Output kinds, nodes, token-bound values, hex colors | CORE |
| `cade-crdt` | Loro project doc, movable layer tree, snapshot save/open, merge, JSON mirror | CORE |
| `cade-policy` | Full `policy.yml` schema and decisions for models, BYOK, shell, network, MCP, design systems, shipping, telemetry, token budgets | POLICY |
| `cade-copilot` | Copilot and BYOK through the Copilot SDK, a policy-enforcing permission handler, change sets, token meter | COPILOT |
| `cade-github` | Device-flow sign-in, keychain storage, github.com / GHE.com / GHES, issues, duplicate search, PRs | GH, FEED |
| `cade-feedback` | Bug / feature / comment → labeled, redacted issue matching the issue forms | FEED |
| `cade-ddl` | DDL for PostgreSQL, SQL Server, MySQL and Oracle; reversible migrations; PII check | DATA |
| `cade-iac` | Bicep and Terraform (azurerm) for App Service, SQL, Key Vault, Storage; missing-setting check | ARCH |
| `cade-system` | Bindings, impact tracing and deploy order across Data → App → Architecture | SYS |
| `cade-io-visio` | `.vsdx` package reader: pages, masters, macro detection | DIAGRAM |
| `cade-io-pptx`, `-figma`, `-spark` | Package/slide reader, Figma link parsing and endpoints, Spark repo scan | IMPORT |
| `cade-render`, `-text`, `-layout` | Vello scene from nodes, Parley text layout, Taffy auto layout | CANVAS |
| `cade-codegen-react` | Design-system TSX emitter | APP |

## Feedback

Use Help › Send feedback in the app, or the issue forms here (bug, feature request, comment). Either way the issue is filed from your own GitHub account. Set `CADE_FEEDBACK_OWNER` at build time to the GitHub account that owns this repo.

## License

Apache-2.0. CADE is an independent project, not affiliated with GitHub or Microsoft. See [NOTICE](NOTICE).
