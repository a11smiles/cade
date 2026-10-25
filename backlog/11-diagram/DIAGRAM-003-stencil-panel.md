---
id: DIAGRAM-003
title: Build the stencil panel with search and drag-to-canvas
tags: [diagram, feature, ui, ts]
epic: DIAGRAM
priority: P0
depends_on: [DIAGRAM-002, DS-016]
design: design/exports/v0001-2026-10-25/Diagram.dc.html
---

## Description

Left panel with page tabs, 'Basic flowchart' grid, 'Imported stencils' (from .vssx) with counts, search
across all libraries, and drag or click to place. Dropping a shape onto a connector splits the connector.

## Acceptance criteria

- [ ] Search finds masters across built-in and imported libraries
- [ ] Drag, click-to-place and keyboard insertion all work
- [ ] Dropping onto a connector inserts the shape and re-glues both halves
- [ ] Matches the Diagram design
