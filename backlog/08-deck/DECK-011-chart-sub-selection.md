---
id: DECK-011
title: Make series, data points and labels individually selectable
tags: [deck, feature, rust]
epic: DECK
priority: P0
depends_on: [DECK-008, CORE-024]
design: design/exports/v0001-2026-10-25/Slides.dc.html
---

## Description

Clicking into a selected chart selects a series, then a single bar or point, then its label. Each
sub-element is a first-class selection target with its own properties (color override, label text,
emphasis) and can be sent to Copilot as context, matching the selectability requirement for images and decks.

## Acceptance criteria

- [ ] Click-through order is chart → series → point → label; Esc walks back up
- [ ] A selected point can get a color override that is stored as a token reference
- [ ] Copilot receives the sub-selection path (chart id, series, index) in its context
- [ ] Hit testing on dense charts selects the nearest mark within 4 px
