---
id: COPILOT-012
title: Persist Copilot threads per project and artifact
tags: [copilot, feature, rust]
epic: COPILOT
priority: P0
depends_on: [COPILOT-002, CORE-015]
design: design/exports/v0001-2026-10-25/Main.dc.html
---

## Description

Each project keeps Copilot threads under `.cade/threads/` (one JSONL file per thread: messages, tool calls, run ids, change-set ids, token usage). A workspace opens its most recent thread (e.g. 'Intake flow thread'). Threads are local files in the project repo but excluded from commits by default via `.gitignore`, with a per-project setting to commit them.

## Acceptance criteria

- [ ] Reopening a project restores the last thread with full message history
- [ ] Thread files are valid JSONL and survive an app crash mid-write (append-only, fsync on run end)
- [ ] Default `.gitignore` excludes `.cade/threads/`; toggling 'Commit Copilot threads' removes the ignore rule
- [ ] Deleting a thread removes its file and its entries from the search index
