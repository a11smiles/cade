---
id: MOTION-002
title: Build the animatable property registry
tags: [motion, feature, rust]
epic: MOTION
priority: P0
depends_on: [MOTION-001, CANVAS-003]
design: design/exports/v0001-2026-10-25/Motion.dc.html
---

## Description

Registry of animatable properties with type and interpolation: position, size, rotation, scale, opacity,
fill and stroke color, corner radius, blur, shadow, path trim (for 'Check · draw'), text reveal and visibility.
Each entry declares whether it can export to Lottie and CSS.

## Acceptance criteria

- [ ] Every listed property can be keyed and plays back on the canvas
- [ ] Each property declares Lottie and CSS export support
- [ ] Unsupported combinations are flagged in the UI before export
- [ ] Registry is covered by unit tests per property type
