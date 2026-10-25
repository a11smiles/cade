---
id: APP-019
title: Play a flow in the app
tags: [app, rust, ts, feature]
epic: APP
priority: P0
depends_on: [APP-016]
design: design/exports/v0001-2026-10-25/Prototype.dc.html
---

## Description

Play flow runs the prototype full-window or in place with hotspot hints, restart, flow picker and device frame. Playback uses the generated code when available, otherwise renders frames from the canvas.

## Acceptance criteria

- [ ] Play starts at the flow's start frame and follows interactions
- [ ] Clicking a non-interactive area flashes hotspot hints
- [ ] Restart and flow picker work during playback
- [ ] Esc exits to the canvas at the current frame
