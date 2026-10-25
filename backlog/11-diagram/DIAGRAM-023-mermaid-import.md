---
id: DIAGRAM-023
title: Import Mermaid diagrams
tags: [diagram, feature, rust]
epic: DIAGRAM
priority: P1
depends_on: [DIAGRAM-022]
design: design/exports/v0001-2026-10-25/Diagram.dc.html
---

## Description

Parse Mermaid flowchart, sequence, state and class diagrams (and treat ER diagrams via the Data model
where possible) into native shapes and connectors, laid out with auto layout. Paste Mermaid text directly
onto the canvas.

## Acceptance criteria

- [ ] Reference Mermaid flowcharts import with matching nodes and edges
- [ ] Pasting Mermaid text creates a diagram
- [ ] Unsupported syntax is reported line by line
