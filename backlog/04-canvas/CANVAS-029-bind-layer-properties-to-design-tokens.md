---
id: CANVAS-029
title: Bind layer properties to design tokens
tags: [canvas, rust, feature]
epic: CANVAS
priority: P0
depends_on: [CORE-007, CANVAS-028]
design: design/exports/v0001-2026-10-25/Image.dc.html
---

## Description

Fills, strokes, text styles, radii, spacing, effects and opacity can reference tokens. Switching theme mode or updating the library re-renders bound layers. Detaching a token keeps the current value.

## Acceptance criteria

- [ ] Binding a fill to a token re-renders when the token value or mode changes
- [ ] Theme mode toggle (dark/light) switches all bound layers in one frame
- [ ] Detach keeps the current raw value and removes the binding
- [ ] Brand-locked documents reject raw colors outside the palette (IMAGE-004)
