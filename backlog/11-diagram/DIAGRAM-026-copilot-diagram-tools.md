---
id: DIAGRAM-026
title: Expose diagram editing as Copilot tools
tags: [diagram, feature, rust]
epic: DIAGRAM
priority: P0
depends_on: [COPILOT-020, DIAGRAM-005]
design: design/exports/v0001-2026-10-25/Diagram.dc.html
---

## Description

Agent tools: add/remove steps, connect shapes, move shapes between lanes, set shape data, generate a
diagram from a description, and rebuild flattened custom shapes as native shapes. Changes are staged.

## Acceptance criteria

- [ ] 'Add the date check' inserts a step and re-glues connectors correctly
- [ ] 'Rebuild the custom shapes natively' replaces vector shapes with editable masters
- [ ] All changes are staged for review
- [ ] Tools are tested with a mocked model
