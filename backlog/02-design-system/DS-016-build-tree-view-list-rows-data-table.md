---
id: DS-016
title: Build tree view, list rows and data table
tags: [ds, feature, ui, a11y, perf]
epic: DS
priority: P0
depends_on: [DS-008]
design: design/exports/v0001-2026-10-25/Main.dc.html
---

## Description

Virtualized tree for layers (drag to reorder, rename inline, visibility/lock toggles), list rows for files and versions, and a table for bindings, settings and imports. Tree and table stay smooth with 10,000 rows.

## Acceptance criteria

- [ ] Tree follows the ARIA treeview pattern
- [ ] Scrolling 10,000 rows holds 60 fps on the reference machine
- [ ] Drag-reorder in the tree emits a single move operation
- [ ] Table supports keyboard row selection and sorting
