---
id: QA-010
title: Enforce performance budgets on 10k-layer files
tags: [qa, test, perf]
epic: QA
priority: P0
depends_on: [QA-009, QA-008]
---

## Description

Budgets on the reference machine for a 10k-layer document: open under 2 s, pan and zoom at 60 fps (no frame over 33 ms), selection feedback under 16 ms, property edit to pixels under 50 ms, memory under 1.5 GB, autosave never blocks input. A benchmark harness records these nightly and fails on regressions over 10%.

## Acceptance criteria

- [ ] Each budget has an automated measurement
- [ ] Nightly results are stored and graphed in the docs site
- [ ] A regression over 10% fails the nightly job and opens an issue
- [ ] Budgets are listed in `docs/perf.md`
