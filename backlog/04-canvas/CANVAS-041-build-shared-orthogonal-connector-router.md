---
id: CANVAS-041
title: Build a shared orthogonal connector router in cade-layout
tags: [canvas, feature, rust, perf]
epic: CANVAS
priority: P1
depends_on: [CORE-005]
---

## Description

Diagram connectors, Azure architecture connections and ER relationship lines all need the same thing: orthogonal paths that avoid obstacles, stay stable as nodes move, and spread parallel lines apart. Build one router in `cade-layout` (visibility-graph or grid-based A* with bend penalties) that takes obstacle rectangles, ports and optional waypoints and returns polylines. DIAGRAM-005, ARCH-009 and DATA-006 consume it with their own port and styling rules.

## Acceptance criteria

- [ ] Given obstacles, a source port and a target port, the router returns an orthogonal polyline that does not cross any obstacle's padded bounds
- [ ] Paths minimize bends first and length second, with a configurable bend penalty
- [ ] Parallel connectors sharing a channel are separated by a configurable gap instead of overlapping
- [ ] Re-routing after moving one node changes only the connectors attached to it or crossing its old and new bounds
- [ ] Routing 500 connectors among 1,000 obstacles completes in under 50 ms on the reference machine
- [ ] Unit tests cover blocked ports, nested containers and user-pinned waypoints

## Notes

Keep the API free of diagram, architecture or data-model types so all three can share it.
