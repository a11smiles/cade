---
id: MOTION-001
title: Define the motion model: timelines, tracks and keyframes
tags: [motion, feature, rust]
epic: MOTION
priority: P0
depends_on: [CORE-003]
design: design/exports/v0001-2026-10-25/Motion.dc.html
---

## Description

Add a `Motion` output type and a reusable timeline object to `cade-core`. A timeline has duration, frame
rate and loop mode; tracks target a layer property path (e.g. `button.width`); keyframes hold time, value
and an easing reference. Durations and easings are stored as token references when bound
(`motion.duration.md`, `motion.ease.out`) so design-system changes flow through.

## Acceptance criteria

- [ ] A timeline with 5 tracks and 20 keyframes round-trips through save and reopen
- [ ] Keyframe times and token references survive concurrent edits
- [ ] Deleting a layer removes or orphans its tracks with a visible warning
- [ ] Timelines can be attached to Motion artifacts and to App components
