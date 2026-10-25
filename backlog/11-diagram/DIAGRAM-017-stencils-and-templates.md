---
id: DIAGRAM-017
title: Import .vssx stencils and .vstx templates
tags: [diagram, feature, rust]
epic: DIAGRAM
priority: P0
depends_on: [DIAGRAM-014, DIAGRAM-003]
design: design/exports/v0001-2026-10-25/Diagram.dc.html
---

## Description

.vssx stencils become shape libraries in the stencil panel ('Contoso BPMN shapes · 24'). .vstx templates
become diagram templates on the Start screen with their pages, styles and stencils.

## Acceptance criteria

- [ ] Imported stencil masters can be placed and keep their connection points
- [ ] Stencils persist in the project and can be shared via the design system
- [ ] Templates create new diagrams with their pages and stencils
