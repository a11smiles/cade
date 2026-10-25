---
id: CANVAS-003
title: Build Vello scenes from the document tree
tags: [canvas, rust, feature, perf]
epic: CANVAS
priority: P0
depends_on: [CORE-003, CANVAS-001]
---

## Description

Translate the Loro layer tree into a Vello scene: frames, rectangles, ellipses, paths, images, text runs, fills, strokes, gradients, clips and opacity groups. Use a retained scene cache per layer keyed by its CRDT version so only changed subtrees are re-encoded.

## Acceptance criteria

- [ ] All primitive layer types render with fills, strokes, gradients, corner radii and clipping
- [ ] Editing one layer re-encodes only that layer's subtree (verified by counters in debug overlay)
- [ ] A 10,000-layer file renders at 60 fps while panning on reference hardware
- [ ] Output matches golden images for a fixture set within a 1% pixel tolerance
