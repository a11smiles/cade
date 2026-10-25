---
id: IMPORT-017
title: Convert Figma auto layout to flex and grid layout
tags: [import, feature, rust]
epic: IMPORT
priority: P0
depends_on: [IMPORT-014, CANVAS-021]
design: design/exports/v0001-2026-10-25/Import.dc.html
---

## Description

Map auto layout direction, spacing, padding, alignment, wrap, hug/fill/fixed sizing, absolute-positioned
children and min/max sizes to Taffy flex or grid layout.

## Acceptance criteria

- [ ] Auto-layout frames resize the same way as in Figma on reference cases
- [ ] Hug, fill and fixed sizing map correctly
- [ ] Absolute children stay absolute
- [ ] Wrap layouts map to flex-wrap
