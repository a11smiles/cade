---
id: CANVAS-040
title: Keep the canvas usable when the GPU is lost
tags: [canvas, rust, feature]
epic: CANVAS
priority: P1
depends_on: [CANVAS-001]
---

## Description

Handle device loss, driver resets and sleep/resume by recreating the wgpu device and rebuilding scene caches without losing document state or selection.

## Acceptance criteria

- [ ] Simulated device loss recovers within 2 s with the same viewport and selection
- [ ] Sleep/resume on all three OSes recovers without a blank canvas
- [ ] Repeated failures switch to the software renderer and notify the user
