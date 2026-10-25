---
id: CANVAS-007
title: Show selection, resize and rotate handles
tags: [canvas, rust, feature, ui]
epic: CANVAS
priority: P0
depends_on: [CANVAS-006]
design: design/exports/v0001-2026-10-25/Main.dc.html
---

## Description

Draw selection outlines, eight resize handles, rotation zones and the dimension badge (e.g. 268 × 46) in the brand violet. Handles stay a constant screen size at every zoom and hide when the selection is too small, offering a move-only target.

## Acceptance criteria

- [ ] Selection outline and handles match the design reference at 25%–6400% zoom
- [ ] Dimension badge shows live width × height while resizing
- [ ] Rotation works from outside corners with 15° snapping on Shift
- [ ] Tiny selections show a single move handle instead of overlapping handles
