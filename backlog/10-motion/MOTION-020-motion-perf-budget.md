---
id: MOTION-020
title: Hold playback performance budgets
tags: [motion, test, perf]
epic: MOTION
priority: P1
depends_on: [MOTION-006]
design: design/exports/v0001-2026-10-25/Motion.dc.html
---

## Description

Automated performance tests for playback and scrubbing on reference scenes (5, 50 and 500 animated layers)
across the three OSes, failing CI on regressions.

## Acceptance criteria

- [ ] 5- and 50-layer scenes hold 60 fps; 500 layers hold 30 fps
- [ ] Scrubbing latency stays under 16 ms for the 50-layer scene
- [ ] CI fails on a regression greater than 10%
