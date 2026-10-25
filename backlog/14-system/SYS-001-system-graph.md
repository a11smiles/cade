---
id: SYS-001
title: Build the linked Data → App → Architecture graph
tags: [sys, feature, rust]
epic: SYS
priority: P0
depends_on: [CORE-022, DATA-001, APP-001, ARCH-001]
design: design/exports/v0001-2026-10-25/Deploy.dc.html
---

## Description

In `cade-system`, maintain a graph of a project's linked artifacts (data model, app, architecture) and the bindings between them, recomputed incrementally when any artifact changes.

## Acceptance criteria

- [ ] Graph links artifacts by stable ids
- [ ] Changes in one artifact mark dependent bindings dirty
- [ ] Graph is recomputed incrementally in under 100 ms for the sample
- [ ] Graph state is saved with the project
