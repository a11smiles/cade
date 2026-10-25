---
id: DIAGRAM-004
title: Implement the connector tool with point and dynamic glue
tags: [diagram, feature, rust]
epic: DIAGRAM
priority: P0
depends_on: [DIAGRAM-001, CANVAS-006]
design: design/exports/v0001-2026-10-25/Diagram.dc.html
---

## Description

Connector tool with straight, right-angle and curved styles. Ends glue to a specific connection point
(static glue) or to the shape (dynamic glue picks the nearest side). Glued connectors follow shapes when
they move, resize or rotate. Hovering shows connection points.

## Acceptance criteria

- [ ] Connectors stay glued when either shape moves
- [ ] Dynamic glue re-picks the side that gives the shortest route
- [ ] Unglued ends are visually distinct and listed in diagram checks
- [ ] Connector style can be switched without losing glue
