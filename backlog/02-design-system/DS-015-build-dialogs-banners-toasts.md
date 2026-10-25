---
id: DS-015
title: Build dialogs, banners and toasts
tags: [ds, feature, ui, a11y]
epic: DS
priority: P0
depends_on: [DS-007]
design: design/exports/v0001-2026-10-25/States.dc.html
---

## Description

Modal dialog with focus trap, non-modal sheet, top-of-workspace banner (used by failure states) and toasts for transient confirmations. Banners and toasts use `role=status` or `role=alert` appropriately.

## Acceptance criteria

- [ ] Dialogs trap focus and restore it on close
- [ ] Banners stay until resolved; toasts auto-dismiss after 6 s and pause on hover
- [ ] Escape closes dialogs unless a destructive action is pending
