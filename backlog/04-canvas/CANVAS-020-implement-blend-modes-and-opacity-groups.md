---
id: CANVAS-020
title: Implement blend modes and opacity groups
tags: [canvas, rust, feature]
epic: CANVAS
priority: P1
depends_on: [CANVAS-019]
---

## Description

Per-layer opacity and blend modes (normal, multiply, screen, overlay, darken, lighten, color dodge/burn, hard/soft light, difference, exclusion, hue, saturation, color, luminosity) rendered through Vello compositing.

## Acceptance criteria

- [ ] All listed blend modes render matching reference images
- [ ] Group opacity applies to the group as a whole, not per child
- [ ] Unsupported blend modes from imports are flagged in the import report
