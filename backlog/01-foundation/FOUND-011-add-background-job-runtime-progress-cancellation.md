---
id: FOUND-011
title: Add a background job runtime with progress and cancellation
tags: [found, feature, rust]
epic: FOUND
priority: P0
depends_on: [FOUND-004]
---

## Description

Run long work (imports, renders, codegen, git operations, agent tool calls) on a tokio runtime behind a `JobManager` that reports progress events to the UI and supports cancellation tokens. Jobs are tied to a project window and are cancelled when it closes.

## Acceptance criteria

- [ ] A job emits typed progress events consumed by a UI progress component
- [ ] Cancelling a job stops work within 500 ms and leaves no partial writes
- [ ] Closing a project window cancels its jobs
- [ ] Panics inside a job are caught and reported as a `CadeError`
