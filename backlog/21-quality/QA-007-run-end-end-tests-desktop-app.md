---
id: QA-007
title: Run end-to-end tests of the desktop app
tags: [qa, test, windows, linux, macos]
epic: QA
priority: P1
depends_on: [QA-005, QA-006]
---

## Description

Use `tauri-driver` with WebDriver on Windows and Linux for full-app tests: first run, create project, edit, apply a Copilot run against a recorded model, ship to a local bare repo. macOS has no WKWebView WebDriver, so run the same scenarios there through a debug-only automation IPC channel and document the gap.

## Acceptance criteria

- [ ] Ten core scenarios run end to end nightly on Windows and Linux
- [ ] The same scenarios run on macOS via the automation channel
- [ ] The automation channel is compiled out of release builds
- [ ] Copilot responses are replayed from recordings so tests are deterministic
