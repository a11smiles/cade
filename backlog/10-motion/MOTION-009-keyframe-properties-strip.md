---
id: MOTION-009
title: Build the keyframe properties strip
tags: [motion, feature, ui, ts]
epic: MOTION
priority: P0
depends_on: [MOTION-007, DS-011]
design: design/exports/v0001-2026-10-25/Motion.dc.html
---

## Description

Strip for the selected keyframe or range: target property, from → to values, duration token, easing token
with curve thumbnail, and 'Ask Copilot about this'.

## Acceptance criteria

- [ ] Values are editable inline with units
- [ ] Duration and easing pickers show tokens first
- [ ] Strip matches the Motion design and wraps at narrow widths
