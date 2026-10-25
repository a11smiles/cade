---
id: DECK-004
title: Add title, body and media placeholders with text autofit
tags: [deck, feature, rust]
epic: DECK
priority: P0
depends_on: [DECK-003, CANVAS-004]
design: design/exports/v0001-2026-10-25/Slides.dc.html
---

## Description

Placeholders hold typed content (title, body, media, chart, table). Empty placeholders show prompt text
that never renders in present mode or exports. Body text supports autofit: shrink font size in token steps,
then reduce line spacing, then flag overflow, never silently clipping.

## Acceptance criteria

- [ ] Empty placeholders show prompt text on canvas but not in present mode, PDF or .pptx
- [ ] Autofit only uses font sizes defined in the type scale tokens
- [ ] Overflowing text that cannot fit shows a red overflow marker and appears in the export check
- [ ] Dropping an image onto a media placeholder crops to fill and keeps the crop editable
- [ ] Autofit can be turned off per placeholder
