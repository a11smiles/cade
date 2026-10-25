---
id: FOUND-001
title: Create the Cargo workspace with all CADE crates
tags: [found, chore, rust]
epic: FOUND
priority: P0
depends_on: []
---

## Description

Set up the root `Cargo.toml` workspace with every crate listed in CONVENTIONS.md as an empty library (`cade-core`, `cade-crdt`, `cade-render`, `cade-text`, `cade-layout`, `cade-copilot`, `cade-policy`, `cade-git`, `cade-github`, `cade-io-*`, `cade-codegen-react`, `cade-iac`, `cade-ddl`, `cade-system`, `cade-feedback`) plus `app/src-tauri` (`cade-app`). Pin the toolchain with `rust-toolchain.toml`, use edition 2024, and define shared `[workspace.dependencies]` and `[workspace.lints]` so every crate inherits the same versions and lint levels.

## Acceptance criteria

- [ ] `cargo check --workspace` passes on macOS, Windows and Linux
- [ ] Each crate has a README line stating its responsibility
- [ ] Shared dependency versions are declared once in `[workspace.dependencies]`
- [ ] `clippy::pedantic` warnings are configured once at workspace level
- [ ] `rust-toolchain.toml` pins a stable version and required components (rustfmt, clippy)
