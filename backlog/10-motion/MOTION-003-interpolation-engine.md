---
id: MOTION-003
title: Implement interpolation for numbers, colors and paths
tags: [motion, feature, rust]
epic: MOTION
priority: P0
depends_on: [MOTION-002]
design: design/exports/v0001-2026-10-25/Motion.dc.html
---

## Description

Evaluate tracks at time t: numeric lerp, color interpolation in OKLab, step/hold keys, path trim, and
compatible-path morphing (same command structure). Deterministic for a given t so export frames are
reproducible.

## Acceptance criteria

- [ ] Color transitions interpolate in OKLab and avoid muddy midpoints (snapshot test)
- [ ] Evaluation at the same t always produces identical output
- [ ] Hold keys keep values constant until the next key
- [ ] Incompatible path morphs are rejected with a clear message
