---
id: MOTION-011
title: Require and preview a reduced-motion fallback
tags: [motion, feature, a11y]
epic: MOTION
priority: P0
depends_on: [MOTION-001]
design: design/exports/v0001-2026-10-25/Motion.dc.html
---

## Description

Every motion has a reduced-motion variant: none, crossfade between first and last state, or a custom
simplified timeline. Header pill shows 'Reduced-motion fallback set'. Preview toggle plays the fallback.
Exports include the fallback (CSS media query, App code path, Lottie alternative).

## Acceptance criteria

- [ ] Header shows a warning pill when no fallback is set
- [ ] Preview toggle plays the fallback variant
- [ ] CSS export wraps the fallback in `prefers-reduced-motion: reduce`
- [ ] App code generation uses the fallback when the OS setting is on
