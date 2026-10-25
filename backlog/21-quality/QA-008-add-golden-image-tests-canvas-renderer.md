---
id: QA-008
title: Add golden-image tests for the canvas renderer
tags: [qa, test, rust]
epic: QA
priority: P0
depends_on: [FOUND-006, CANVAS-003]
---

## Description

Render fixture documents offscreen with Vello and compare against reference PNGs with a perceptual threshold. Cover fills, strokes, gradients, effects, text shaping, clipping, blend modes and zoom levels. Run on a software adapter in CI for determinism, and on real GPUs nightly.

## Acceptance criteria

- [ ] Golden tests cover every node kind that renders
- [ ] CI uses a deterministic software adapter
- [ ] Nightly runs on Metal, D3D12 and Vulkan report differences
- [ ] Updating goldens produces a visual diff report for review
