---
id: IMAGE-012
title: Check text legibility across sizes
tags: [image, rust, feature, a11y]
epic: IMAGE
priority: P1
depends_on: [IMAGE-003, CANVAS-037]
design: design/exports/v0001-2026-10-25/Image.dc.html
---

## Description

Flag sizes where text falls below a minimum readable size or contrast after scaling (e.g. headline in the email header), and suggest a per-size override.

## Acceptance criteria

- [ ] Text below a configurable minimum px at a size is flagged
- [ ] Contrast is checked per size against the rendered background
- [ ] Flags offer a one-click per-size override or a Copilot fix
