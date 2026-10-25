---
id: DECK-009
title: Build the chart properties strip
tags: [deck, feature, ui, ts]
epic: DECK
priority: P0
depends_on: [DECK-008, DS-011]
design: design/exports/v0001-2026-10-25/Slides.dc.html
---

## Description

When a chart is selected the contextual strip shows: chart name, type switcher (bar, line, donut, more),
series color swatches limited to design-system tokens, 'Edit values', label placement, gridlines, and
'Ask Copilot about this'. Raw hex colors are only available if org policy allows off-token colors.

## Acceptance criteria

- [ ] Type switch, series color and label placement apply immediately and are undoable
- [ ] Series swatches list only tokens unless policy allows custom colors
- [ ] 'Ask Copilot about this' opens the Copilot input with the chart as the context chip
- [ ] Strip matches the Slides design and wraps at narrow widths
