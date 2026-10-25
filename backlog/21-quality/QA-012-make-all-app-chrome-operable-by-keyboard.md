---
id: QA-012
title: Make all app chrome operable by keyboard only
tags: [qa, test, a11y]
epic: QA
priority: P0
depends_on: [QA-006]
---

## Description

Walk every v0001 screen with keyboard only: tool rails, panels, properties strip, Copilot composer, menus, dialogs, settings and admin. Document a focus order and shortcut map per workspace, and fix every trap or unreachable control.

## Acceptance criteria

- [ ] Every interactive control in every screen is reachable and operable by keyboard
- [ ] Focus is always visible
- [ ] A shortcut moves focus between rail, panels, canvas, strip and Copilot
- [ ] Checklist per screen is committed in `docs/a11y/`
