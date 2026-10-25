---
id: FOUND-004
title: Generate typed IPC bindings between Rust and TypeScript
tags: [found, feature, rust, ts]
epic: FOUND
priority: P0
depends_on: [FOUND-003]
---

## Description

Use tauri-specta (or ts-rs plus a thin wrapper) so every `#[tauri::command]` and event payload has a generated TypeScript type in `app/ui/src/ipc/bindings.ts`. Commands return a shared `CadeError` type so the UI can render consistent error states. Generation runs in a build step and CI fails if bindings are stale.

## Acceptance criteria

- [ ] Adding a command and running `just bindings` updates `bindings.ts`
- [ ] CI fails when committed bindings differ from generated ones
- [ ] Events (progress, document changes, agent updates) are typed end to end
- [ ] A sample command round-trips a struct with enums and optional fields
