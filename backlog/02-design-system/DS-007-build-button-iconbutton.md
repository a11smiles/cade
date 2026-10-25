---
id: DS-007
title: Build Button and IconButton
tags: [ds, feature, ui, a11y]
epic: DS
priority: P0
depends_on: [DS-002, DS-005, DS-006]
---

## Description

Variants: primary (violet), secondary (bordered), ghost, danger, success (Approve), and the tinted Copilot variant ("Ask Copilot about this"). Sizes 26/30/34/40/44 px. Real `<button>` elements with visible focus rings, disabled and loading states. Link-styled buttons render as `<a>`.

## Acceptance criteria

- [ ] All variants and sizes render in dark and light
- [ ] Keyboard focus ring meets 3:1 contrast against adjacent colors
- [ ] Loading state keeps width stable and announces progress to screen readers
- [ ] IconButton requires an accessible name at the type level
