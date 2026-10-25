---
id: DS-014
title: Build menus, context menus and the command palette
tags: [ds, feature, ui, a11y]
epic: DS
priority: P1
depends_on: [DS-007, FOUND-008]
---

## Description

Dropdown menus (model picker, export menus), right-click context menus on canvas and layers (bridged from native events), and a Cmd/Ctrl+K command palette that searches commands, layers, files and recent prompts. Shortcuts shown come from the shared registry.

## Acceptance criteria

- [ ] Menus follow the ARIA menu pattern with typeahead
- [ ] Canvas right-click opens a context menu at the pointer
- [ ] Palette finds commands, layers and files with fuzzy search under 50 ms on 10k layers
- [ ] Palette shows each command's shortcut
