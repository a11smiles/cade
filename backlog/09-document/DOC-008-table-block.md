---
id: DOC-008
title: Add the table block shared by documents and decks
tags: [doc, feature, rust]
epic: DOC
priority: P0
depends_on: [DOC-002]
design: design/exports/v0001-2026-10-25/Document.dc.html
---

## Description

Tables with header rows, column widths (fixed, auto, percent), cell merge, per-cell alignment and text
styles, zebra striping from tokens, and row splitting across pages with repeated headers. Shared component
used by Deck (DECK-012).

## Acceptance criteria

- [ ] Tables split across pages by row and repeat header rows
- [ ] Column widths can be fixed, auto or percent
- [ ] Cell merge and split are undoable
- [ ] Tables export to native Word and PowerPoint tables
