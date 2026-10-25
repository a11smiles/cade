---
id: CANVAS-002
title: Route input from the webview and OS to the canvas
tags: [canvas, rust, ts, feature]
epic: CANVAS
priority: P0
depends_on: [CANVAS-001]
---

## Description

Define one input pipeline in `cade-render` for pointer, wheel, trackpad pinch/rotate, pen pressure and keyboard events. Events over the canvas come from the native window; keyboard focus is shared with the webview through an explicit focus owner so shortcuts don't double-fire.

## Acceptance criteria

- [ ] Pointer, wheel and pinch events arrive with canvas-space coordinates and modifiers
- [ ] Pen pressure and tilt are delivered on macOS, Windows Ink and Linux (libinput) where available
- [ ] Only one of webview or canvas owns keyboard focus at a time; Tab moves focus between them
- [ ] Input latency from OS event to rendered frame is under 16 ms at 60 Hz on reference hardware
