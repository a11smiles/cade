---
id: IMPORT-006
title: Convert React/TypeScript screens into App frames
tags: [import, feature, rust]
epic: IMPORT
priority: P0
depends_on: [IMPORT-005, APP-005]
design: design/exports/v0001-2026-10-25/Import.dc.html
---

## Description

Parse TSX with SWC, find routes and top-level screens, and build App frames and layers from the JSX tree.
Map known components to design-system components where they match and keep unknown components as
generic layers bound to source code so two-way sync still works.

## Acceptance criteria

- [ ] Reference Spark apps produce one frame per screen
- [ ] Known components map to design-system components
- [ ] Unmapped components remain linked to their source files
- [ ] The App workspace can edit imported frames and write code back

## Notes

Absorbs APP-027, which duplicated this item and was removed during backlog reconciliation.
