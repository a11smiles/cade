---
id: CANVAS-027
title: Build the Tokens panel
tags: [canvas, ts, ui, feature]
epic: CANVAS
priority: P1
depends_on: [CORE-007, DS-010]
design: design/exports/v0001-2026-10-25/Main.dc.html
---

## Description

The Tokens tab lists color, type, spacing, radius, effect and motion tokens from the linked design system with values per theme mode, and shows where each token is used in the file.

## Acceptance criteria

- [ ] Tokens are grouped by type and show values for each mode (dark/light)
- [ ] Clicking 'Find usages' selects every layer bound to the token
- [ ] Tokens from read-only org libraries cannot be edited here; local tokens can
