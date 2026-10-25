---
id: MOTION-017
title: Export CSS keyframes that reference design tokens
tags: [motion, feature, rust]
epic: MOTION
priority: P1
depends_on: [MOTION-004, MOTION-011]
design: design/exports/v0001-2026-10-25/Motion.dc.html
---

## Description

Generate `@keyframes` and transition CSS. Durations and easings are emitted as CSS custom properties
named after motion tokens (e.g. `var(--motion-duration-sm)`), with the reduced-motion fallback in a media
query. Option to emit Web Animations API JavaScript.

## Acceptance criteria

- [ ] Generated CSS reproduces the animation in Chrome, Safari and Firefox
- [ ] Token-bound values use CSS custom properties
- [ ] Reduced-motion fallback is included
- [ ] Web Animations output is available as an option
