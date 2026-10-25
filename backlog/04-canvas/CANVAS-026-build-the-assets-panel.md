---
id: CANVAS-026
title: Build the Assets panel
tags: [canvas, ts, ui, feature]
epic: CANVAS
priority: P0
depends_on: [CORE-009, CORE-008, DS-010]
design: design/exports/v0001-2026-10-25/Main.dc.html
---

## Description

The Assets tab lists components from the project and linked design systems, images, and icons, searchable and grouped. Drag an asset onto the canvas to create an instance or image layer.

## Acceptance criteria

- [ ] Components from the linked design system (e.g. Contoso DS v4.2) are listed with previews
- [ ] Search filters components, images and icons by name and tag
- [ ] Dragging a component onto the canvas creates an instance at the drop point
- [ ] The panel shows the linked library version and an update badge when a new version exists
