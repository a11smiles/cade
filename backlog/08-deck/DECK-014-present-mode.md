---
id: DECK-014
title: Implement full-screen present mode
tags: [deck, feature, rust]
epic: DECK
priority: P0
depends_on: [DECK-008, CANVAS-003]
design: design/exports/v0001-2026-10-25/Slides.dc.html
---

## Description

Present renders slides full-screen on the chosen display through the GPU renderer at native resolution.
Keyboard, clicker and touch navigation; black/white screen keys; simple transitions (none, fade, push)
honoring reduced-motion; live linked frames render their latest version.

## Acceptance criteria

- [ ] Present starts on the current slide within 300 ms and runs at the display refresh rate
- [ ] Arrow keys, Page Up/Down, Space, B and W behave as in PowerPoint
- [ ] Hidden slides are skipped
- [ ] Transitions are disabled when the OS reduced-motion setting is on
- [ ] Works on macOS, Windows and Linux with multiple displays
