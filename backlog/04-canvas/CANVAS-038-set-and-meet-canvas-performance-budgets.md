---
id: CANVAS-038
title: Set and meet canvas performance budgets
tags: [canvas, rust, test, perf]
epic: CANVAS
priority: P0
depends_on: [CANVAS-003, CANVAS-010]
---

## Description

Define and enforce budgets in CI with benchmark fixtures: open a 10,000-layer file in under 2 s, 60 fps pan/zoom, under 1 ms hit test, under 100 ms for a typical Copilot change set to apply and render, memory under 1.5 GB.

## Acceptance criteria

- [ ] Benchmark fixtures exist for small, medium and 10,000-layer files
- [ ] CI runs benchmarks on all three OSes and fails on a >10% regression
- [ ] A debug overlay shows frame time, encode time and GPU memory
