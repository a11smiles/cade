---
id: FOUND-021
title: Run CI on macOS, Windows and Linux for every PR
tags: [found, chore, rust, ts]
epic: FOUND
priority: P0
depends_on: [FOUND-001, FOUND-002]
---

## Description

GitHub Actions workflow with a matrix of `macos-latest`, `windows-latest` and `ubuntu-latest`: rustfmt, clippy with `-D warnings`, `cargo test`, UI typecheck/lint/test, binding freshness, and a debug Tauri build. Cache cargo and pnpm stores. Actions are pinned by SHA.

## Acceptance criteria

- [ ] Every PR runs the full matrix and blocks merge on failure
- [ ] Warm-cache CI completes in under 20 minutes
- [ ] All third-party actions are pinned to commit SHAs
- [ ] Workflow permissions default to `contents: read`
