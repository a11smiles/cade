---
id: MOTION-005
title: Build the timeline panel with tracks, ruler and bars
tags: [motion, feature, ui, ts]
epic: MOTION
priority: P0
depends_on: [MOTION-001, DS-010]
design: design/exports/v0001-2026-10-25/Motion.dc.html
---

## Description

Bottom timeline panel: track list on the left, time ruler, per-track bars spanning key ranges, zoom on
the time axis, and selection that syncs with the canvas. Matches the Motion design.

## Acceptance criteria

- [ ] Selecting a track selects its layer on the canvas and vice versa
- [ ] Time axis zooms from 100 ms to 60 s visible range
- [ ] Track rows can be reordered, renamed, muted and soloed
- [ ] Panel is resizable and remembers its height
