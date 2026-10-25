---
id: CORE-020
title: Provide a git merge driver for .cade projects
tags: [core, feature, rust]
epic: CORE
priority: P1
depends_on: [CORE-019, CORE-002]
---

## Description

Register a custom merge driver via `.gitattributes` so `doc.loro` and journal files merge through the CRDT instead of producing binary conflicts. The CLI exposes `cade merge-driver` for use outside the app. The JSON export is regenerated after merge.

## Acceptance criteria

- [ ] Two branches with independent edits merge with no conflict markers
- [ ] Same-property conflicts resolve deterministically and are flagged in the app
- [ ] Driver works when invoked by command-line git on all three OSes
- [ ] `cade init` writes the needed `.gitattributes` entries

## Notes

Absorbs GH-022, which duplicated this item and was removed during backlog reconciliation.
