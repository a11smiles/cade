---
id: APP-017
title: Add transitions using motion tokens
tags: [app, rust, feature]
epic: APP
priority: P1
depends_on: [APP-016, DS-005]
design: design/exports/v0001-2026-10-25/Prototype.dc.html
---

## Description

Transitions (instant, dissolve, slide, push, smart animate) use motion tokens for duration and easing (e.g. motion.slide · 250ms) and respect reduced-motion settings in Play.

## Acceptance criteria

- [ ] Each transition type plays in Play mode
- [ ] Duration and easing pick from motion tokens
- [ ] With reduced motion on, transitions fall back to a crossfade or instant
