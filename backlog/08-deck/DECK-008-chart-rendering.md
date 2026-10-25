---
id: DECK-008
title: Render charts on the canvas with axes, labels and legends
tags: [deck, feature, rust]
epic: DECK
priority: P0
depends_on: [DECK-007, CANVAS-003]
design: design/exports/v0001-2026-10-25/Slides.dc.html
---

## Description

Render every chart kind with Vello: axes with tick generation, optional gridlines, data labels on or
outside marks, legends, and token-driven colors. Rendering must be deterministic so thumbnails, PNG export
and .pptx fallback images match.

## Acceptance criteria

- [ ] All seven chart kinds render with correct proportions for a reference dataset (snapshot tests)
- [ ] Gridlines, axis titles, legend and data labels can be toggled independently
- [ ] Changing a color token re-renders affected charts without re-layout of the slide
- [ ] Charts with 1,000 points render in under 8 ms on the reference machine
