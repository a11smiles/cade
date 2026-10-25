---
id: CANVAS-018
title: Add the eyedropper
tags: [canvas, rust, feature]
epic: CANVAS
priority: P1
depends_on: [CANVAS-003]
design: design/exports/v0001-2026-10-25/Image.dc.html
---

## Description

Pick colors from anywhere on the canvas, including images, and optionally from the screen where the OS allows. When the picked color matches a design token within tolerance, offer the token instead of a raw value.

## Acceptance criteria

- [ ] Eyedropper samples the rendered canvas pixel under the pointer
- [ ] Screen-wide sampling works on macOS and Windows; Linux uses the desktop portal
- [ ] Picked colors that match a token within ΔE 2 suggest the token
- [ ] Esc cancels without changing the fill
