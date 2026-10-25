---
id: DECK-021
title: Read .pptx and .potx into the deck model
tags: [deck, feature, rust]
epic: DECK
priority: P0
depends_on: [DECK-003, DECK-007]
design: design/exports/v0001-2026-10-25/Slides.dc.html
---

## Description

`cade-io-pptx` reader: parse masters, layouts, themes, slides, text frames, preset and custom geometry,
pictures, tables, charts (with embedded data), notes and section markers into the deck model. A .potx becomes
a reusable master. Unsupported content (SmartArt, OLE objects, embedded video) is converted to the nearest
equivalent and recorded in the import fidelity report.

## Acceptance criteria

- [ ] A 24-slide reference deck with 6 charts imports with editable charts and correct data
- [ ] Theme colors and fonts become tokens, matched to the active design system where close
- [ ] SmartArt imports as grouped shapes and is reported as approximated
- [ ] Embedded video becomes a link to the original file and is reported
- [ ] Malformed files fail with a readable error and never crash the app (fuzz-tested)

## Notes

The import UI and pipeline integration are in IMPORT-022; this item is the parser and mapping only.
