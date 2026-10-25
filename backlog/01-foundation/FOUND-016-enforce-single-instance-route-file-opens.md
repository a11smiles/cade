---
id: FOUND-016
title: Enforce single instance and route file opens
tags: [found, feature, rust]
epic: FOUND
priority: P1
depends_on: [FOUND-007]
---

## Description

Use the Tauri single-instance plugin so launching CADE again, or double-clicking a file, focuses the running app and opens the target in a new or existing window.

## Acceptance criteria

- [ ] Second launch focuses the existing instance
- [ ] Opening a `.cade` file from the OS opens or focuses its project window
- [ ] Works from Finder, Explorer and common Linux file managers
