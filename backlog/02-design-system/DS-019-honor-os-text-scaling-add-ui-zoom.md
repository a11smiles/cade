---
id: DS-019
title: Honor OS text scaling and add UI zoom
tags: [ds, feature, a11y]
epic: DS
priority: P1
depends_on: [DS-010]
---

## Description

Respect OS text scaling and offer UI zoom (80%–150%) independent of canvas zoom. Layouts reflow without clipping at 150% on a 1280×800 window.

## Acceptance criteria

- [ ] UI zoom is in Settings and has shortcuts
- [ ] No clipped text on key screens at 150%
- [ ] Canvas zoom is unaffected by UI zoom
