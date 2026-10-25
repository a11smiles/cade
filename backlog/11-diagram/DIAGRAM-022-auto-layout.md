---
id: DIAGRAM-022
title: Add automatic diagram layout
tags: [diagram, feature, rust]
epic: DIAGRAM
priority: P1
depends_on: [DIAGRAM-005]
design: design/exports/v0001-2026-10-25/Diagram.dc.html
---

## Description

Layered (Sugiyama-style) layout for flowcharts with direction (top-down, left-right), lane-aware layout
for swimlanes, and tidy-tree layout for org charts. Used by Mermaid import, Copilot-generated diagrams and a
'Tidy up' command.

## Acceptance criteria

- [ ] Layered layout produces no overlaps and minimizes crossings on reference graphs
- [ ] Lane-aware layout keeps shapes in their lanes
- [ ] 'Tidy up' is a single undo step
