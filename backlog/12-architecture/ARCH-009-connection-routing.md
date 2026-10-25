---
id: ARCH-009
title: Route architecture connections orthogonally around resources
tags: [arch, feature, rust, perf]
epic: ARCH
priority: P1
depends_on: [ARCH-008, CANVAS-041]
design: design/exports/v0001-2026-10-25/Architecture.dc.html
---

## Description

Use an orthogonal router that avoids resource tiles and container labels, keeps connections glued to tile edges, and re-routes live while dragging. Share the router with the Diagram and Data model canvases.

## Acceptance criteria

- [ ] Routes avoid overlapping tiles and labels in a 50-resource test scene
- [ ] Dragging a tile re-routes attached lines at 60 fps
- [ ] Users can add bend points; manual bends survive small moves
- [ ] Parallel connections between the same pair fan out instead of overlapping

## Notes

Shared with DIAGRAM connector routing and DATA ER routing; implement once in cade-layout.
