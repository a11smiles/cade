---
id: DIAGRAM-011
title: Support text on shapes with wrap and autosize
tags: [diagram, feature, rust]
epic: DIAGRAM
priority: P0
depends_on: [DIAGRAM-001, CANVAS-005]
design: design/exports/v0001-2026-10-25/Diagram.dc.html
---

## Description

Double-click to edit text inside shapes and on connectors. Text wraps to the shape's text box, can
autosize the shape, and uses design-system text styles.

## Acceptance criteria

- [ ] Text wraps inside the master's text box
- [ ] Autosize grows the shape height to fit text
- [ ] Text styles come from the design system
