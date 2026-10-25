---
id: COPILOT-001
title: Create cade-copilot crate wrapping github-copilot-sdk
tags: [copilot, feature, rust]
epic: COPILOT
priority: P0
depends_on: [FOUND-001]
---

## Description

Create the `cade-copilot` crate as the only place in CADE that talks to a model. It wraps the `github-copilot-sdk` Rust crate's client (which drives the bundled Copilot CLI in server mode over JSON-RPC) behind CADE-owned types so the rest of the app never imports SDK types directly. Expose an async API built on Tokio: `connect()`, `create_session(project, options)`, `send(session, message)` returning a stream of events, `cancel(run)`, and `close()`.

## Acceptance criteria

- [ ] `cargo test -p cade-copilot` runs against a mock transport with no network and passes
- [ ] No crate other than `cade-copilot` depends on `github-copilot-sdk` (enforced by a `cargo deny` or workspace lint check in CI)
- [ ] A session can be created, sent one text message, streamed to completion and closed in an integration test gated behind `CADE_LIVE_TESTS=1`
- [ ] SDK errors are mapped to a CADE `CopilotError` enum (Auth, Policy, Network, ModelUnavailable, Budget, Tool, Cancelled, Internal)
- [ ] Public API is documented with rustdoc and `#![deny(missing_docs)]` passes

## Notes

Pin the SDK version in the workspace `Cargo.toml`; bump deliberately with a changelog entry because the CLI protocol is versioned with it.
