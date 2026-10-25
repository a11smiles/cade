---
id: CANVAS-013
title: Implement the Frame tool and nested frames
tags: [canvas, rust, feature]
epic: CANVAS
priority: P0
depends_on: [CANVAS-008]
design: design/exports/v0001-2026-10-25/Main.dc.html
---

## Description

The Frame tool draws frames that clip content, can nest, and appear as top-level artboards with name and size labels (e.g. 'Step 1 · Mobile 390×720'). Offer device presets (phone, tablet, desktop, social sizes) in the properties strip.

## Acceptance criteria

- [ ] Drawing a frame around existing layers adopts them as children
- [ ] Top-level frames show name and size labels on the canvas
- [ ] Clip content can be toggled per frame
- [ ] Device and size presets set the frame size in one click
