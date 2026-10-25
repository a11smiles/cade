---
id: HIST-008
title: Branch from any version and switch branches
tags: [hist, history, feature, rust, ui]
epic: HIST
priority: P1
depends_on: [HIST-004, GH-007]
design: design/exports/v0001-2026-10-25/History.dc.html
---

## Description

'Branch from here' creates `cade/from-<sha>` and opens it. A branch switcher in the header lists CADE branches; switching saves current work first. Merging branches uses the CRDT merge of the two documents and records a merge commit.

## Acceptance criteria

- [ ] New branch is created at the selected commit and opened
- [ ] Switching branches never loses unsaved work
- [ ] Merging two branches with concurrent edits produces a combined document without conflicts
- [ ] Branch list shows ahead/behind counts
