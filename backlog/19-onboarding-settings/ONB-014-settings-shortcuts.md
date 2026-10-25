---
id: ONB-014
title: Implement the keyboard shortcuts viewer and editor
tags: [onb, feature, ui]
epic: ONB
priority: P1
depends_on: [ONB-008]
design: design/exports/v0001-2026-10-25/Settings.dc.html
---

## Description

Searchable list of all commands with their shortcuts per platform, rebinding with conflict detection, and presets (CADE default, Figma-style, Visio-style). Export/import as JSON.

## Acceptance criteria

- [ ] All registered commands appear with platform-correct modifiers
- [ ] Rebinding to an existing shortcut shows the conflict and offers swap
- [ ] Presets switch all bindings at once
- [ ] Reset to defaults works per command and globally
