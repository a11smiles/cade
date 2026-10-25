---
id: CANVAS-010
title: Pan and zoom the infinite canvas
tags: [canvas, rust, feature, perf]
epic: CANVAS
priority: P0
depends_on: [CANVAS-002]
---

## Description

Pan with space-drag, middle mouse, trackpad two-finger scroll; zoom with Ctrl/Cmd+wheel, pinch, Z tool, shortcuts (zoom to fit, to selection, 100%). Zoom is anchored at the pointer. The zoom percentage in the header is editable.

## Acceptance criteria

- [ ] Pan and zoom are smooth at 60 fps on a 10,000-layer file
- [ ] Zoom anchors at the pointer for wheel and pinch
- [ ] Shift+1 fits all, Shift+2 fits selection, Cmd/Ctrl+0 goes to 100%
- [ ] Typing a value in the header zoom field sets the zoom
