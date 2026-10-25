---
id: DIAGRAM-005
title: Build orthogonal connector routing with obstacle avoidance
tags: [diagram, feature, rust, perf]
epic: DIAGRAM
priority: P0
depends_on: [DIAGRAM-004, CANVAS-041]
design: design/exports/v0001-2026-10-25/Diagram.dc.html
---

## Description

Router for right-angle connectors that avoids shapes, keeps a minimum clearance, minimizes bends, adds
line jumps at crossings (optional) and re-routes only affected connectors on edits. Users can add manual
waypoints that the router respects.

## Acceptance criteria

- [ ] Connectors never pass through unrelated shapes at default clearance
- [ ] Moving one shape re-routes only its connectors and neighbors in under 8 ms for a 500-shape page
- [ ] Line jumps can be turned on per page
- [ ] Manual waypoints persist and survive re-routing
