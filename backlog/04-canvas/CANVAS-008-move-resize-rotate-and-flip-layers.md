---
id: CANVAS-008
title: Move, resize, rotate and flip layers
tags: [canvas, rust, feature]
epic: CANVAS
priority: P0
depends_on: [CANVAS-007, CORE-012]
---

## Description

Implement transforms with Shift (proportional/axis lock), Alt (from center) and arrow-key nudging (1 px, 10 px with Shift). Resizing honors each child's constraints and text resizing mode. Every gesture commits one undo step.

## Acceptance criteria

- [ ] Drag, resize, rotate and flip horizontal/vertical work on single and multiple layers
- [ ] Shift and Alt modifiers behave as specified; nudging works with arrow keys
- [ ] Resizing a frame applies children's constraints (CANVAS-022)
- [ ] Each completed gesture is exactly one undo step
