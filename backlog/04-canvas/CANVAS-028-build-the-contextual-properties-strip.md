---
id: CANVAS-028
title: Build the contextual properties strip
tags: [canvas, ts, ui, feature]
epic: CANVAS
priority: P0
depends_on: [CANVAS-007, CORE-007, DS-011]
design: design/exports/v0001-2026-10-25/Main.dc.html
---

## Description

The floating bottom strip shows properties for the current selection: instance name, layout (W/H/gap), fill, text style, radius, variant, accessibility status, and 'Ask Copilot about this'. Values bound to tokens display the token name (e.g. color.action.primary); editing offers tokens first.

## Acceptance criteria

- [ ] Strip content changes with selection type (instance, frame, text, shape, image, multi)
- [ ] Token-bound values show token names; raw values show a 'not a token' marker
- [ ] Editing a field updates the canvas live and commits one undo step on blur
- [ ] Contrast badge (e.g. AA 5.9:1) recalculates for text and fills
- [ ] The strip wraps instead of overflowing at narrow window widths
