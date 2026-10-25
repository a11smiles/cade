---
id: CANVAS-021
title: Lay out frames with Taffy auto layout
tags: [canvas, rust, feature]
epic: CANVAS
priority: P0
depends_on: [CANVAS-013, CORE-004]
design: design/exports/v0001-2026-10-25/Main.dc.html
---

## Description

Integrate `cade-layout` on Taffy for flex and grid auto layout: direction, gap, padding, alignment, wrap, hug/fill/fixed sizing, min/max sizes, absolute-positioned children, and grid tracks. Layout recomputes incrementally on CRDT changes.

## Acceptance criteria

- [ ] Flex direction, gap, padding, alignment and wrap behave like CSS flexbox
- [ ] Children can be hug, fill or fixed on each axis with min/max
- [ ] Grid layout supports track counts and spans
- [ ] Changing one child's text re-lays out only affected ancestors
- [ ] Layout output matches the generated React/CSS (APP-008) for a fixture set
