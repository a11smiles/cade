---
id: MOTION-010
title: Show key states side by side on the stage
tags: [motion, feature, ui]
epic: MOTION
priority: P1
depends_on: [MOTION-006]
design: design/exports/v0001-2026-10-25/Motion.dc.html
---

## Description

Stage view that shows named states (e.g. Idle 0.00 s, Loading 0.48 s, Success 1.20 s) as side-by-side
snapshots with the playhead state highlighted. States are markers on the timeline and can be renamed.

## Acceptance criteria

- [ ] Markers on the timeline appear as state snapshots on the stage
- [ ] Clicking a snapshot moves the playhead to that state
- [ ] Snapshots re-render when keyframes change
