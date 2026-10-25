---
id: CANVAS-024
title: Group, ungroup, reorder and align layers
tags: [canvas, rust, feature]
epic: CANVAS
priority: P0
depends_on: [CANVAS-009]
---

## Description

Group/ungroup, bring forward/backward, align (left, center, right, top, middle, bottom), distribute spacing, tidy up, and wrap in flex or grid. All operations work on multi-selections and are single undo steps.

## Acceptance criteria

- [ ] Group and ungroup preserve positions and transforms
- [ ] Align and distribute work relative to selection or parent
- [ ] Reordering updates z-order in the layers panel and CRDT tree
- [ ] 'Wrap in flex' and 'Wrap in grid' create an auto layout frame around the selection
