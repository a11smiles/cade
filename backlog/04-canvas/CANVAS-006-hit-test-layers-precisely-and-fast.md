---
id: CANVAS-006
title: Hit-test layers precisely and fast
tags: [canvas, rust, feature, perf]
epic: CANVAS
priority: P0
depends_on: [CANVAS-003]
---

## Description

Maintain a spatial index (R-tree) of layer bounds updated from CRDT changes, and do exact hit tests on paths, strokes and text glyph boxes. Respect lock, hidden, clipping and frame boundaries; deep-select with Cmd/Ctrl.

## Acceptance criteria

- [ ] Clicks hit the topmost visible, unlocked layer under the pointer, including thin strokes (≥ 3 px tolerance at any zoom)
- [ ] Cmd/Ctrl-click selects the deepest layer; plain click selects the outermost group or frame child
- [ ] Hit test on a 10,000-layer file completes in under 1 ms
- [ ] Hidden and locked layers are never hit
