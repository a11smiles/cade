---
id: DIAGRAM-020
title: Export diagrams to .vsdx
tags: [diagram, feature, rust]
epic: DIAGRAM
priority: P0
depends_on: [DIAGRAM-014]
design: design/exports/v0001-2026-10-25/Diagram.dc.html
---

## Description

`cade-io-visio` writer producing valid .vsdx: pages, masters (embedded), shapes with geometry, text,
styles, connectors with glue (Connects), layers, shape data, swimlanes and background pages. Shapes that
came from Visio keep their original ShapeSheet cells where unchanged.

## Acceptance criteria

- [ ] Exported files open without repair in Visio desktop and Visio for the web
- [ ] Glue is preserved: moving a shape in Visio drags its connectors
- [ ] Shape data and layers appear in Visio
- [ ] CADE-native diagrams export with editable shapes, not images
