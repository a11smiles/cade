---
id: MOTION-004
title: Support cubic-bezier and spring easing bound to tokens
tags: [motion, feature, rust]
epic: MOTION
priority: P0
depends_on: [MOTION-003, DS-005]
design: design/exports/v0001-2026-10-25/Motion.dc.html
---

## Description

Easing functions: cubic-bezier, linear, steps and spring (mass/stiffness/damping, converted to a sampled
curve for export). Design systems define motion tokens for durations and easings. Keys bound to tokens show
the token name; custom values are marked 'off-token'.

## Acceptance criteria

- [ ] Design-system motion tokens appear in duration and easing pickers
- [ ] Changing a token value updates every bound keyframe
- [ ] Off-token values are visually flagged and listed by the policy check if the org requires tokens
- [ ] Spring easing exports as a sampled curve to CSS and Lottie
