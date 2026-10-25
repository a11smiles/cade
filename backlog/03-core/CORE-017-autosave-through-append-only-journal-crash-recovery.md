---
id: CORE-017
title: Autosave through an append-only journal with crash recovery
tags: [core, feature, rust]
epic: CORE
priority: P0
depends_on: [CORE-015, FOUND-011]
---

## Description

Every transaction is appended to the journal within 500 ms. A background job compacts the journal into `doc.loro` when it grows past a threshold or on close. On launch after a crash the journal is replayed, so at most the last half second of work can be lost.

## Acceptance criteria

- [ ] Killing the process mid-edit loses at most 500 ms of changes
- [ ] Compaction runs off the UI thread and never blocks input
- [ ] A corrupted trailing journal entry is skipped and reported, not fatal
- [ ] Journal files are fsync'd on macOS, Windows and Linux
