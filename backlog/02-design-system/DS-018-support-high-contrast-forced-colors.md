---
id: DS-018
title: Support high contrast and forced colors
tags: [ds, feature, a11y, windows]
epic: DS
priority: P1
depends_on: [DS-003]
---

## Description

Respect Windows forced-colors mode and `prefers-contrast: more` with a high-contrast token set. Focus indicators, selection outlines and status pills remain visible.

## Acceptance criteria

- [ ] Windows High Contrast themes render all chrome legibly
- [ ] `prefers-contrast: more` swaps to the high-contrast token set
- [ ] Native canvas overlays switch to system highlight colors in forced-colors mode
