---
id: CANVAS-025
title: Build the Layers panel
tags: [canvas, ts, ui, feature]
epic: CANVAS
priority: P0
depends_on: [CORE-004, DS-010, DS-016]
design: design/exports/v0001-2026-10-25/Main.dc.html
---

## Description

The left panel's Layers tab shows pages and the layer tree with icons by type, component badges, visibility and lock toggles, rename on double-click, drag to reorder and reparent, and virtualized rendering for large trees. Selection syncs both ways with the canvas.

## Acceptance criteria

- [ ] Selecting in the panel selects on the canvas and vice versa
- [ ] Drag-and-drop reorders and reparents layers with a drop indicator
- [ ] Visibility and lock toggles work and are undoable
- [ ] A 10,000-layer tree scrolls at 60 fps
- [ ] Copilot-generating layers show a pending indicator
