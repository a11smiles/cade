---
id: DOC-012
title: Support multi-column sections
tags: [doc, feature, rust]
epic: DOC
priority: P2
depends_on: [DOC-002]
design: design/exports/v0001-2026-10-25/Document.dc.html
---

## Description

Sections can have 1–3 columns with gutter tokens. Content balances across columns on the last page of a
section. Blocks can span columns.

## Acceptance criteria

- [ ] Two- and three-column sections lay out text correctly
- [ ] Column balancing works on the last page of a section
- [ ] A block can be set to span all columns
