---
id: COPILOT-004
title: Manage the bundled Copilot CLI process lifecycle
tags: [copilot, feature, rust, windows, macos, linux]
epic: COPILOT
priority: P0
depends_on: [COPILOT-001, FOUND-003]
---

## Description

The Rust SDK bundles the Copilot CLI binary. CADE must locate it inside the app bundle on each OS, start it in server mode on demand, health-check it, restart it after a crash with backoff, and shut it down with the app. The CLI's working directory is always the open project folder so file tools stay scoped.

## Acceptance criteria

- [ ] CLI starts lazily on the first Copilot request, not at app launch
- [ ] Killing the CLI process externally causes an automatic restart and the next prompt succeeds
- [ ] Three crashes within 60 s stop restarts and show the 'Copilot runtime unavailable' state
- [ ] No orphaned CLI process remains after quitting CADE on macOS, Windows and Linux (CI check)
- [ ] CLI version and binary hash are shown in Settings › About
