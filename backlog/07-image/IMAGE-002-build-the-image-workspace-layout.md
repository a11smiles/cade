---
id: IMAGE-002
title: Build the Image workspace layout
tags: [image, ts, ui, feature]
epic: IMAGE
priority: P0
depends_on: [IMAGE-001, DS-010]
design: design/exports/v0001-2026-10-25/Image.dc.html
---

## Description

IMAGE header tag, project name and size count, brand-lock pill, zoom and 'Export N sizes'. Left panel shows Sizes (kept in sync) and Layers; tool rail has select, shapes, pen, text, place image, brand assets, eyedropper and crop.

## Acceptance criteria

- [ ] Layout matches the design reference in dark and light
- [ ] Selecting a size in the Sizes list focuses that artboard
- [ ] Tool rail tools map to CANVAS tools with Image-specific defaults
