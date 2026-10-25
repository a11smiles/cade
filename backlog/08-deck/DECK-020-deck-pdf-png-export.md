---
id: DECK-020
title: Export decks to PDF, handouts and PNG
tags: [deck, feature, rust]
epic: DECK
priority: P1
depends_on: [DECK-014, CANVAS-032]
design: design/exports/v0001-2026-10-25/Slides.dc.html
---

## Description

PDF export of slides (one per page), handouts (1/3/6 per page with notes lines) and notes pages, plus PNG
per slide at 1×/2×/4×. Uses `cade-io-export` so output matches present mode exactly.

## Acceptance criteria

- [ ] PDF slides match present-mode rendering pixel-for-pixel at 2× (snapshot test)
- [ ] Handout layouts 1, 3 and 6 per page are available
- [ ] PNG export supports selected slides only
- [ ] Text in PDF is selectable and searchable
