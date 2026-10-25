---
id: DECK-001
title: Define the deck and slide model in the Loro document
tags: [deck, feature, rust]
epic: DECK
priority: P0
depends_on: [CORE-003]
design: design/exports/v0001-2026-10-25/Slides.dc.html
---

## Description

Add a `Deck` output type to `cade-core`: an ordered list of slides (Loro MovableList), each slide a frame
whose children are regular layers, plus a reference to a layout and its master. Store deck-level settings
(aspect ratio, default master, section markers) on the deck node. Slides must reuse the same layer, component
and token machinery as every other output so that any element on a slide is selectable and editable.

## Acceptance criteria

- [ ] A deck with 12 slides round-trips through save, close and reopen with order, layout refs and children intact
- [ ] Reordering slides from two concurrent CRDT peers converges to the same order
- [ ] Every slide child is a standard layer node that the selection model and inspector accept
- [ ] Deck settings (aspect ratio, default master) are stored once on the deck node, not per slide
- [ ] Unit tests cover insert, delete, move and duplicate of slides
