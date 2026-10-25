---
id: DIAGRAM-013
title: Build the diagram properties strip
tags: [diagram, feature, ui, ts]
epic: DIAGRAM
priority: P0
depends_on: [DIAGRAM-007, DIAGRAM-010, DS-011]
design: design/exports/v0001-2026-10-25/Diagram.dc.html
---

## Description

Strip for selected shape or connector: kind and text, fill, connector style and glue status, lane, shape
data summary and 'Ask Copilot'. Fill shows tokens first and raw hex where the diagram came from Visio.

## Acceptance criteria

- [ ] Strip content adapts to shape vs connector selection
- [ ] Imported raw colors show as hex with a 'map to token' action
- [ ] All controls are keyboard accessible
