---
id: CANVAS-012
title: Add rulers, guides and layout grids
tags: [canvas, rust, ts, feature]
epic: CANVAS
priority: P1
depends_on: [CANVAS-011]
---

## Description

Show toggleable rulers; drag guides out of rulers; per-frame layout grids (columns, rows, grid) with gutter, margin and count. Guides and grids are stored in the document and snap targets.

## Acceptance criteria

- [ ] Rulers show units relative to the selected frame
- [ ] Guides can be dragged from rulers, moved, and deleted
- [ ] Column, row and square grids are configurable per frame and render over content
- [ ] Layers snap to guides and grid lines
