---
id: DIAGRAM-015
title: Evaluate a ShapeSheet formula subset and freeze the rest
tags: [diagram, feature, rust]
epic: DIAGRAM
priority: P1
depends_on: [DIAGRAM-014]
design: design/exports/v0001-2026-10-25/Diagram.dc.html
---

## Description

Evaluate the ShapeSheet cells and functions needed for geometry and layout (Width, Height, PinX/PinY,
LocPin, Angle, geometry rows, basic arithmetic, GUARD, IF, references to sibling cells). Formulas outside the
subset are frozen at their stored value and listed as 'formula frozen' in the fidelity report.

## Acceptance criteria

- [ ] Resizing an imported shape whose geometry uses supported formulas reshapes correctly
- [ ] Unsupported formulas keep their last value and are listed by cell
- [ ] Exported .vsdx writes back original formulas for frozen cells untouched
