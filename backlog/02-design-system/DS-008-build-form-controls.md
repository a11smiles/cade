---
id: DS-008
title: Build form controls
tags: [ds, feature, ui, a11y]
epic: DS
priority: P0
depends_on: [DS-002, DS-005]
---

## Description

Text input, textarea, select, combobox with search, checkbox, radio group, toggle, slider and the token-value field (shows a swatch plus token name like `color.action.primary`). Every control has a `<label>` association, error text and a description slot.

## Acceptance criteria

- [ ] Each control works with keyboard only
- [ ] Errors are announced and linked via `aria-describedby`
- [ ] Token-value field shows swatch, token name and raw value on hover
- [ ] Controls render in dark and light without contrast failures
