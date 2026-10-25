---
id: MOTION-015
title: Export GIF with palette quantization
tags: [motion, feature, rust]
epic: MOTION
priority: P1
depends_on: [MOTION-013]
design: design/exports/v0001-2026-10-25/Motion.dc.html
---

## Description

GIF export with per-animation palette generation, optional dithering, frame-rate cap, loop count and a
size estimate before export.

## Acceptance criteria

- [ ] Exports loop correctly in browsers and Slack
- [ ] A size estimate is shown before export
- [ ] Dithering can be turned off for flat UI animations
