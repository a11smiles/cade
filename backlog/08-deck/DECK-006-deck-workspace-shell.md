---
id: DECK-006
title: Build the Deck workspace header and layout
tags: [deck, feature, ui, ts]
epic: DECK
priority: P0
depends_on: [DECK-002, DS-010, COPILOT-014]
design: design/exports/v0001-2026-10-25/Slides.dc.html
---

## Description

Workspace chrome for Deck: DECK type tag, deck title and slide count, master and size picker, Present
button, Export menu, slide strip, stage with the selected slide, contextual properties strip, speaker notes
panel and the Copilot column bound to a deck thread. Matches the exported design in dark and light themes.

## Acceptance criteria

- [ ] Layout matches the Slides design at 1440×900 in both themes
- [ ] Copilot panel opens on the deck's own thread and shows the current selection as a context chip
- [ ] Export menu lists .pptx, PDF and PNG per slide
- [ ] All controls are reachable by keyboard with visible focus
