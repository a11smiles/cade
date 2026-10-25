# CADE architecture

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

See `docs/adr/` for the decisions behind this.
