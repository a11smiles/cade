---
id: IMPORT-014
title: Convert Figma frames, groups and vectors to native layers
tags: [import, feature, rust]
epic: IMPORT
priority: P0
depends_on: [IMPORT-013, CANVAS-003]
design: design/exports/v0001-2026-10-25/Import.dc.html
---

## Description

Map frames, groups, rectangles, ellipses, vectors (vector networks to paths), boolean operations, text,
images, fills (solid, gradient, image), strokes, corner radii and effects (shadows, blur) to CADE layers.

## Acceptance criteria

- [ ] Reference file renders within 1 px of Figma's rendered PNGs on 95% of frames
- [ ] Boolean operations remain editable
- [ ] Image fills keep their scale mode
- [ ] Unsupported blend modes are flattened and reported
