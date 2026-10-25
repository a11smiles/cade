---
id: HIST-010
title: Build a static review bundle for a version
tags: [hist, history, feature, rust]
epic: HIST
priority: P1
depends_on: [HIST-004, CANVAS-032]
design: design/exports/v0001-2026-10-25/Review.dc.html
---

## Description

Generate a static review site for a version: rendered frames (PNG + SVG), prototype flow as a lightweight HTML player, frame list and metadata. No source design data or Copilot threads are included.

## Acceptance criteria

- [ ] Bundle contains rendered frames and a working prototype player
- [ ] Bundle size for a 20-frame project is under 20 MB
- [ ] No `.cade`, thread or prompt data is present in the bundle (test)
- [ ] Bundle opens offline from disk for local preview
