---
id: CANVAS-039
title: Add canvas keyboard shortcuts and the tool rail
tags: [canvas, ts, feature, a11y]
epic: CANVAS
priority: P0
depends_on: [CANVAS-002, DS-007, FOUND-008]
design: design/exports/v0001-2026-10-25/Main.dc.html
---

## Description

Wire the tool rail (select, frame, rectangle, ellipse, pen, text, image, components, comment) with single-key shortcuts (V, F, R, O, P, T, Shift+Cmd+K, C) and platform-correct modifiers. Shortcuts are listed in Settings › Keyboard shortcuts and customizable.

## Acceptance criteria

- [ ] Every rail tool has a shortcut and a tooltip showing it
- [ ] Shortcuts use Cmd on macOS and Ctrl on Windows/Linux
- [ ] Shortcuts don't fire while typing in a text field or the Copilot input
- [ ] Rail buttons are reachable by keyboard and have accessible names
