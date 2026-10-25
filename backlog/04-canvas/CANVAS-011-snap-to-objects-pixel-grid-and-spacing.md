---
id: CANVAS-011
title: Snap to objects, pixel grid and spacing
tags: [canvas, rust, feature]
epic: CANVAS
priority: P0
depends_on: [CANVAS-008]
---

## Description

Snap moving and resizing layers to edges and centers of nearby layers, the pixel grid, layout guides and equal spacing. Draw red/violet snap lines and spacing pills while dragging. Hold Ctrl to disable snapping temporarily.

## Acceptance criteria

- [ ] Edges and centers snap within 6 screen pixels to siblings and the parent frame
- [ ] Equal-spacing snapping shows distance pills between three or more layers
- [ ] Layers land on whole pixels unless pixel snapping is turned off
- [ ] Holding Ctrl/Cmd during a drag disables snapping
