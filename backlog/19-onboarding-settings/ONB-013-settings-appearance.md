---
id: ONB-013
title: Implement Appearance with dark, light and system themes
tags: [onb, feature, ui, a11y]
epic: ONB
priority: P0
depends_on: [ONB-008, DS-003]
design: design/exports/v0001-2026-10-25/Settings.dc.html
---

## Description

Theme picker with Dark, Light and Match system previews. Match system follows OS appearance changes live on macOS, Windows and Linux (via Tauri theme events). Also UI density (comfortable/compact), reduced motion (follow OS or force), and canvas background preference.

## Acceptance criteria

- [ ] Switching theme updates every open window without restart
- [ ] Match system follows OS changes within 1 s
- [ ] Reduced motion follows the OS setting by default
- [ ] Theme choice persists and applies before first paint (no flash)
