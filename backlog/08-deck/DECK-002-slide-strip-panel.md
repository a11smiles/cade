---
id: DECK-002
title: Build the slide strip with reorder, duplicate and sections
tags: [deck, feature, ui, ts]
epic: DECK
priority: P0
depends_on: [DECK-001, DS-010]
design: design/exports/v0001-2026-10-25/Slides.dc.html
---

## Description

Left panel in the Deck workspace showing numbered slide thumbnails rendered by the canvas renderer at low
resolution. Supports drag to reorder, multi-select, duplicate, delete, hide slide, and named sections that
collapse. "+ Slide from layout" opens a layout picker filtered to the current master.

## Acceptance criteria

- [ ] Thumbnails update within 500 ms of an edit on the slide
- [ ] Drag reorder works with mouse and with keyboard (Alt+Up/Down) and is undoable
- [ ] Hidden slides show a hidden badge and are skipped in present mode
- [ ] Sections can be added, renamed and collapsed; collapsing does not change slide order
- [ ] Panel meets keyboard and screen reader requirements (each thumbnail announces slide number and title)
