---
id: IMAGE-006
title: Apply effects with presets
tags: [image, rust, feature]
epic: IMAGE
priority: P0
depends_on: [CANVAS-019]
design: design/exports/v0001-2026-10-25/Image.dc.html
---

## Description

Effect presets (glow, soft shadow, long shadow, blur) map to effect stacks with editable radius and strength, and can be bound to effect tokens.

## Acceptance criteria

- [ ] Glow preset renders as configured (e.g. 60 / 35%)
- [ ] Presets expand to editable effect stacks
- [ ] Effects can bind to design system effect tokens
