---
id: DIAGRAM-001
title: Define the diagram model: shapes, connectors, pages and layers
tags: [diagram, feature, rust]
epic: DIAGRAM
priority: P0
depends_on: [CORE-003]
design: design/exports/v0001-2026-10-25/Diagram.dc.html
---

## Description

Add a `Diagram` output type to `cade-core`: pages, shapes (instances of masters with geometry, text and
style), connectors with begin/end glue (to a connection point or to the shape as a whole), containers,
swimlanes, layers and per-shape data. The model must map cleanly to Visio's concepts so .vsdx import and
export can round-trip.

## Acceptance criteria

- [ ] A two-page diagram with 40 shapes, 30 connectors and 3 layers round-trips through save and reopen
- [ ] Connector glue references survive moving and renaming shapes
- [ ] Concurrent moves of a shape and its connector converge
- [ ] The model has a documented mapping table to Visio elements (Shape, Master, Connect, Layer, Property)
