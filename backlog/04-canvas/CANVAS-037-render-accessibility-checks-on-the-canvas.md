---
id: CANVAS-037
title: Render accessibility checks on the canvas
tags: [canvas, rust, feature, a11y]
epic: CANVAS
priority: P1
depends_on: [CANVAS-028]
design: design/exports/v0001-2026-10-25/Main.dc.html
---

## Description

Compute contrast for text against its rendered background, minimum touch target size, and missing labels on interactive components, and show issues as canvas badges and in the strip.

## Acceptance criteria

- [ ] Contrast is computed from the rendered background, including images and gradients
- [ ] Touch targets under 44×44 px on interactive components are flagged
- [ ] Interactive components without an accessible name are flagged
- [ ] Issues link to the layer and can be fixed via 'Ask Copilot'
