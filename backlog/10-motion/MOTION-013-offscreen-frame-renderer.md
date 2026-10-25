---
id: MOTION-013
title: Render deterministic offscreen frames for export
tags: [motion, feature, rust]
epic: MOTION
priority: P0
depends_on: [MOTION-003, CANVAS-003]
design: design/exports/v0001-2026-10-25/Motion.dc.html
---

## Description

Headless wgpu + Vello render path that produces RGBA frames at any resolution and frame rate without a
window, used by MP4, GIF and PNG-sequence export and by CI snapshot tests.

## Acceptance criteria

- [ ] Renders 1080p frames headless on macOS, Windows and Linux CI runners
- [ ] Output is bit-identical across runs on the same platform
- [ ] Supports transparent backgrounds
- [ ] Renders 120 frames at 1080p in under 5 seconds on the reference machine
