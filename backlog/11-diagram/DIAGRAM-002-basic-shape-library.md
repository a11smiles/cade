---
id: DIAGRAM-002
title: Ship built-in flowchart and process shape libraries
tags: [diagram, feature, rust]
epic: DIAGRAM
priority: P0
depends_on: [DIAGRAM-001]
design: design/exports/v0001-2026-10-25/Diagram.dc.html
---

## Description

Built-in masters: basic flowchart (process, decision, start/end, data, document, predefined process,
reference, manual input), basic shapes, BPMN core (task, events, gateways) and org chart. Each master
defines geometry, connection points, text box and default style tokens.

## Acceptance criteria

- [ ] The stencil panel lists all built-in libraries
- [ ] Every master has connection points on each side at minimum
- [ ] Masters render crisply at all zoom levels
- [ ] Masters use design-system tokens for default fill, stroke and text
