---
id: DS-003
title: Switch between Dark, Light and Match system themes
tags: [ds, feature, ui, rust]
epic: DS
priority: P0
depends_on: [DS-002]
design: design/exports/v0001-2026-10-25/Settings.dc.html
---

## Description

Add the Appearance setting from the Settings screen. Match system follows the OS live (no restart). The theme applies to the webview, native canvas overlays, window title bar (macOS appearance, Windows Mica/dark title bar, GTK dark preference) and app icons where the OS supports it. Design content on the canvas is never re-themed.

## Acceptance criteria

- [ ] Changing the theme updates all open windows within one frame
- [ ] Match system reacts to OS theme changes while the app is running
- [ ] Title bars match the theme on all three OSes
- [ ] Canvas content colors are unaffected by the app theme
- [ ] Theme choice persists across restarts
