---
id: MOTION-012
title: Use motions in App prototype interactions
tags: [motion, feature, rust]
epic: MOTION
priority: P0
depends_on: [MOTION-001, APP-016]
design: design/exports/v0001-2026-10-25/Motion.dc.html
---

## Description

Attach a motion to an App component state change or prototype transition ('Use in App prototype'). The
prototype player and React code generation both use the same timeline, so what plays in CADE matches the
shipped app.

## Acceptance criteria

- [ ] A motion can be bound to a component state change from the Motion and App workspaces
- [ ] Prototype playback uses the motion timeline
- [ ] Generated React code reproduces the timing within one frame
- [ ] Editing the motion updates every binding
