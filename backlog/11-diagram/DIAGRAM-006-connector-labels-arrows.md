---
id: DIAGRAM-006
title: Add connector labels and arrowheads
tags: [diagram, feature, rust]
epic: DIAGRAM
priority: P0
depends_on: [DIAGRAM-004]
design: design/exports/v0001-2026-10-25/Diagram.dc.html
---

## Description

Text labels on connectors (e.g. Yes/No) positioned along the path and draggable, plus begin/end arrowheads
(none, arrow, open, filled, diamond, crow's foot) and line patterns (solid, dashed, dotted).

## Acceptance criteria

- [ ] Labels stay attached when the connector re-routes
- [ ] All listed arrowhead types render and export to .vsdx
- [ ] Dashed lines render consistently on canvas and in exports
