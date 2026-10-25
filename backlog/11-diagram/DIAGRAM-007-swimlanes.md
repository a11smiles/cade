---
id: DIAGRAM-007
title: Implement swimlanes with lane membership
tags: [diagram, feature, rust]
epic: DIAGRAM
priority: P0
depends_on: [DIAGRAM-001]
design: design/exports/v0001-2026-10-25/Diagram.dc.html
---

## Description

Cross-functional swimlanes (horizontal or vertical) with titled lanes. Shapes dropped in a lane become
members; resizing a lane moves its members; lanes can be added, reordered and removed. The properties strip
shows and changes a shape's lane.

## Acceptance criteria

- [ ] Dropping a shape into a lane sets its lane membership
- [ ] Changing lane in the strip moves the shape into that lane
- [ ] Reordering lanes moves their members
- [ ] Swimlanes import from and export to Visio cross-functional flowcharts
