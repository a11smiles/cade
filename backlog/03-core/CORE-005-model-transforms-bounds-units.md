---
id: CORE-005
title: Model transforms, bounds and units
tags: [core, feature, rust]
epic: CORE
priority: P0
depends_on: [CORE-004]
---

## Description

Each node stores a local affine transform, size and constraints. A derived cache provides absolute transforms and axis-aligned bounds, invalidated on change. Units are CSS px for screens, pt for print documents, with conversion helpers.

## Acceptance criteria

- [ ] Absolute bounds are correct for nested rotated and scaled frames (property tests)
- [ ] Cache invalidation updates only the changed subtree
- [ ] px/pt/in/mm conversions are exact for print sizes
- [ ] Bounds queries on 10k nodes complete under 2 ms
