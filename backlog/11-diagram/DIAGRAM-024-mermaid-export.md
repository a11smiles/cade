---
id: DIAGRAM-024
title: Export diagrams to Mermaid
tags: [diagram, feature, rust]
epic: DIAGRAM
priority: P1
depends_on: [DIAGRAM-001]
design: design/exports/v0001-2026-10-25/Diagram.dc.html
---

## Description

Export flowchart-like pages to Mermaid flowchart syntax with node shapes, labels, edge labels and
subgraphs for lanes and containers. Layout-only details are dropped and noted.

## Acceptance criteria

- [ ] Exported Mermaid renders on github.com
- [ ] Lanes become subgraphs
- [ ] A notice lists dropped styling
