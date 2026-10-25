---
id: FOUND-020
title: Provide one-command developer setup
tags: [found, chore]
epic: FOUND
priority: P0
depends_on: [FOUND-001, FOUND-002]
---

## Description

Add a `justfile` (or `cargo xtask`) with `setup`, `dev`, `test`, `lint`, `bindings` and `bundle` tasks, plus a devcontainer for Linux contributors. Document OS prerequisites (Xcode CLT, MSVC + WebView2, WebKitGTK 4.1 and libsoup3).

## Acceptance criteria

- [ ] `just setup && just dev` launches the app from a clean clone on all three OSes
- [ ] Prerequisites per OS are in `CONTRIBUTING.md`
- [ ] Devcontainer builds and runs tests headlessly
