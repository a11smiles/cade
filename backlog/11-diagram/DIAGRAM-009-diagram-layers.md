---
id: DIAGRAM-009
title: Support layers with visibility, lock and print settings
tags: [diagram, feature, rust]
epic: DIAGRAM
priority: P0
depends_on: [DIAGRAM-001]
design: design/exports/v0001-2026-10-25/Diagram.dc.html
---

## Description

Layers with name, color, visible, locked, printable and snap/glue flags. A shape can belong to several
layers (as in Visio). The left panel lists layers with a visibility toggle.

## Acceptance criteria

- [ ] Hidden layers do not render or export unless 'include hidden' is chosen
- [ ] Locked layers cannot be selected on canvas
- [ ] Multi-layer membership round-trips with .vsdx
- [ ] Layer panel matches the Diagram design
