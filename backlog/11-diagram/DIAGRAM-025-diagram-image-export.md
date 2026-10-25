---
id: DIAGRAM-025
title: Export diagram pages to SVG, PDF and PNG
tags: [diagram, feature, rust]
epic: DIAGRAM
priority: P0
depends_on: [DIAGRAM-001, CANVAS-032]
design: design/exports/v0001-2026-10-25/Diagram.dc.html
---

## Description

Export the current page, selected pages or selection to SVG (with text as text), PDF (vector, one page per
diagram page) and PNG (1×/2×/4×, transparent option) via `cade-io-export`.

## Acceptance criteria

- [ ] SVG keeps text selectable and connectors as paths
- [ ] PDF pages match canvas rendering
- [ ] PNG supports transparent backgrounds
