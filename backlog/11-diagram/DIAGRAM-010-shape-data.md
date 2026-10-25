---
id: DIAGRAM-010
title: Implement shape data (custom properties)
tags: [diagram, feature, rust]
epic: DIAGRAM
priority: P0
depends_on: [DIAGRAM-001]
design: design/exports/v0001-2026-10-25/Diagram.dc.html
---

## Description

Per-shape typed properties (string, number, date, boolean, list) such as Owner, SLA, Rule ID. Masters can
define default properties. Shown in the strip, editable in a data panel, and available to Copilot as
context.

## Acceptance criteria

- [ ] Properties can be added, edited and removed per shape
- [ ] Masters supply default property definitions
- [ ] Shape data round-trips with Visio Shape Data
- [ ] Copilot context includes the selected shape's data
