---
id: IMPORT-001
title: Build the shared import pipeline
tags: [import, feature, rust]
epic: IMPORT
priority: P0
depends_on: [CORE-011, GH-001]
design: design/exports/v0001-2026-10-25/Import.dc.html
---

## Description

Define an `ImportSource` trait and pipeline in `cade-core`: discover → stage (copy inputs read-only into a
staging folder) → convert → validate → write a new project folder → `git init` and first commit
('Imported from …'). Supports progress events, cancellation and batch import. Originals are opened read-only
and never modified.

## Acceptance criteria

- [ ] Every importer runs through the same pipeline and emits progress events
- [ ] Cancelling leaves no partial project folder
- [ ] Original files and repos are opened read-only; a test asserts their hashes are unchanged
- [ ] The new project starts as a git repo with an 'Imported from' commit
- [ ] Importers run in a separate process so a crash cannot take down the app
