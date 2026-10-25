---
id: CANVAS-019
title: Add fills, strokes and effects
tags: [canvas, rust, feature]
epic: CANVAS
priority: P0
depends_on: [CANVAS-003]
design: design/exports/v0001-2026-10-25/Image.dc.html
---

## Description

Support multiple fills and strokes per layer (solid, linear/radial/angular/diamond gradients, image), stroke alignment, dash patterns, caps and joins, and effects: drop shadow, inner shadow, layer blur, background blur, glow (as shadow preset).

## Acceptance criteria

- [ ] Layers can hold multiple ordered fills and strokes
- [ ] All gradient types render and are editable on the canvas with handles
- [ ] Stroke alignment, dashes, caps and joins match SVG semantics
- [ ] Drop/inner shadow, layer blur and background blur render correctly and are exportable
