---
id: DIAGRAM-008
title: Add containers with membership and auto-resize
tags: [diagram, feature, rust]
epic: DIAGRAM
priority: P1
depends_on: [DIAGRAM-001]
design: design/exports/v0001-2026-10-25/Diagram.dc.html
---

## Description

Containers group shapes visually with a heading and optional auto-resize to fit members. Moving a
container moves members. Containers nest.

## Acceptance criteria

- [ ] Shapes dropped inside become members; dragging out removes membership
- [ ] Auto-resize keeps a padding token around members
- [ ] Nested containers move as a unit
