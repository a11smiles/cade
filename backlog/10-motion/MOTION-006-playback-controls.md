---
id: MOTION-006
title: Add playhead, play/pause, loop and frame stepping
tags: [motion, feature, rust]
epic: MOTION
priority: P0
depends_on: [MOTION-003, MOTION-005]
design: design/exports/v0001-2026-10-25/Motion.dc.html
---

## Description

Playback driven by the render loop: play/pause (Space), loop toggle, scrub by dragging the playhead,
frame step (comma/period), time readout (00:00.48 / 00:01.20) and frame-rate selection (24/30/60).

## Acceptance criteria

- [ ] Playback holds 60 fps for the reference animation on the reference machine
- [ ] Scrubbing updates the canvas at interactive rates
- [ ] Loop and frame step behave as specified
- [ ] Time readout matches the evaluated frame exactly
