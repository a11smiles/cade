---
id: MOTION-016
title: Export Lottie JSON
tags: [motion, feature, rust]
epic: MOTION
priority: P1
depends_on: [MOTION-002, MOTION-004]
design: design/exports/v0001-2026-10-25/Motion.dc.html
---

## Description

Write Lottie (Bodymovin) JSON for supported layer types and properties: shapes, fills, strokes, trim
paths, transforms, opacity and text as shapes. Unsupported properties are listed before export. Validate
output against the Lottie schema and the lottie-web player.

## Acceptance criteria

- [ ] Reference animation exports and plays identically in lottie-web (frame-compare test)
- [ ] Unsupported properties are listed and can be baked to keyframes
- [ ] Output validates against the Lottie JSON schema
- [ ] Easing curves map to Lottie bezier handles
