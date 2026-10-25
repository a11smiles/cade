---
id: FOUND-007
title: Support multiple project windows and persist window state
tags: [found, feature, rust, ui]
epic: FOUND
priority: P1
depends_on: [FOUND-003]
---

## Description

Each open project gets its own window; Settings and Admin open as separate windows. Persist size, position, maximized state and the last workspace per project, and restore them on relaunch while clamping to visible displays.

## Acceptance criteria

- [ ] Opening two projects shows two independent windows
- [ ] Window geometry is restored after relaunch
- [ ] A window saved on a disconnected monitor reopens on the primary display
- [ ] Closing the last window on macOS keeps the app running; on Windows/Linux it quits
