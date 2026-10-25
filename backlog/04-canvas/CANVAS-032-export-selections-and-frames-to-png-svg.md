---
id: CANVAS-032
title: Export selections and frames to PNG, SVG and PDF
tags: [canvas, rust, feature]
epic: CANVAS
priority: P0
depends_on: [CANVAS-003]
---

## Description

Export settings per layer (format, scale 0.5x–4x, suffix) using `cade-io-export`. PNG and JPEG render through Vello offscreen; SVG writes vector output with text as text or outlines; PDF embeds fonts and keeps vectors.

## Acceptance criteria

- [ ] Export produces PNG/JPEG at 1x, 2x and 3x with correct pixel dimensions
- [ ] SVG output re-imports into CADE and browsers with matching appearance
- [ ] PDF output keeps vectors and embeds subsetted fonts
- [ ] Batch export writes all marked layers in one action
