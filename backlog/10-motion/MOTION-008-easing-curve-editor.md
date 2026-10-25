---
id: MOTION-008
title: Build the easing curve editor
tags: [motion, feature, ui]
epic: MOTION
priority: P1
depends_on: [MOTION-004]
design: design/exports/v0001-2026-10-25/Motion.dc.html
---

## Description

Inline curve editor in the properties strip showing the bezier curve with draggable handles, a preset
list from the design system, and the bound token name. Editing a handle detaches from the token unless the
user saves a new token (if they have design-system edit rights).

## Acceptance criteria

- [ ] Handles drag with live preview on the canvas
- [ ] Presets list design-system easings by token name
- [ ] Detaching from a token is explicit and reversible
- [ ] Curve editor is operable by keyboard with numeric inputs
