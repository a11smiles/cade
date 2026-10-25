---
id: ONB-006
title: Choose the projects folder
tags: [onb, feature, ui, rust, macos, windows, linux]
epic: ONB
priority: P0
depends_on: [ONB-001]
design: design/exports/v0001-2026-10-25/Onboarding.dc.html
---

## Description

Projects folder input (default `~/design` / `%USERPROFILE%\design`) with a native folder picker. Validate write access, create if missing, and on macOS request security-scoped bookmark access so the app can reopen it after restart.

## Acceptance criteria

- [ ] Default path is OS-appropriate
- [ ] Choose… opens the native picker
- [ ] Non-writable folders are rejected with a reason
- [ ] macOS sandbox access persists across restarts
- [ ] Folder is editable later in Settings › General
