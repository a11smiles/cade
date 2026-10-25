---
id: QA-009
title: Generate large documents for performance testing
tags: [qa, test, perf, rust]
epic: QA
priority: P0
depends_on: [CORE-003]
---

## Description

A generator that produces realistic large projects: 10k and 50k layers, 500 component instances with overrides, 200 text nodes with mixed styles, 50 pages, a 300-shape Visio import, an 80-resource Azure design and a 150-entity data model.

## Acceptance criteria

- [ ] `just perf-corpus` generates all sizes reproducibly from a seed
- [ ] Corpus files are not committed; they're generated in CI
- [ ] Generator covers every output type
