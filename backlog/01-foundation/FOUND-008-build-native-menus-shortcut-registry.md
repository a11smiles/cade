---
id: FOUND-008
title: Build native menus and a shortcut registry
tags: [found, feature, rust, ui, macos, windows, linux]
epic: FOUND
priority: P1
depends_on: [FOUND-003]
---

## Description

Provide platform-correct menus: the macOS app menu (About, Settings, Services, Hide, Quit) and File/Edit/View/Window/Help menus on all platforms. Back them with one shortcut registry shared with the command palette so a shortcut is defined once and shown everywhere, using Cmd on macOS and Ctrl elsewhere.

## Acceptance criteria

- [ ] Menus follow each OS's conventions (macOS app menu, Windows/Linux menubar)
- [ ] Shortcuts are defined in one registry and appear in menus and the palette
- [ ] Conflicting shortcut registrations fail at startup in debug builds
- [ ] Edit menu Undo/Redo route to the document undo stack, not the webview
