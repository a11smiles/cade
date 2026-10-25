---
id: IMPORT-022
title: Import PowerPoint decks and templates through the pipeline
tags: [import, feature, rust]
epic: IMPORT
priority: P0
depends_on: [IMPORT-001, DECK-021]
design: design/exports/v0001-2026-10-25/Import.dc.html
---

## Description

Wire .pptx and .potx into the pipeline using the deck reader: dry run produces mapping and warnings
(SmartArt as grouped shapes, embedded video linked), theme colors matched to design-system tokens, and the
result opens in the Deck workspace.

## Acceptance criteria

- [ ] A deck and template import together and the template becomes the deck's master
- [ ] Theme colors are matched to tokens and differences reported
- [ ] The success state opens the Deck workspace
