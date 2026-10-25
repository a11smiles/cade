---
id: DIAGRAM-019
title: Show the Visio import fidelity report in the Diagram workspace
tags: [diagram, feature, ui]
epic: DIAGRAM
priority: P0
depends_on: [DIAGRAM-014, IMPORT-002]
design: design/exports/v0001-2026-10-25/Diagram.dc.html
---

## Description

'Visio import' panel in the right column: file name, editability status, and checks such as pages and
lanes, shapes imported as native, connector glue kept, shape data and layers, custom shapes converted to
vectors, ShapeSheet formulas frozen, and supported formats note.

## Acceptance criteria

- [ ] Report counts match the importer's results
- [ ] Each warning selects the affected shapes when clicked
- [ ] Report is stored with the project and viewable later
