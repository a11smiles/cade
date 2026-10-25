---
id: DECK-012
title: Add tables to slides
tags: [deck, feature, rust]
epic: DECK
priority: P1
depends_on: [DECK-001, DOC-008]
design: design/exports/v0001-2026-10-25/Slides.dc.html
---

## Description

Reuse the document table block on slides: header row styling from tokens, column resize, cell merge,
and per-cell text styles. Tables export to native PowerPoint tables.

## Acceptance criteria

- [ ] Tables can be inserted into body placeholders or freely on a slide
- [ ] Column resize and cell merge are undoable
- [ ] Tables export as editable PowerPoint tables
- [ ] Table text uses design-system text styles
