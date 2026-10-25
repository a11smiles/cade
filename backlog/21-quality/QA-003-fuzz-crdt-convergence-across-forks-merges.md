---
id: QA-003
title: Fuzz CRDT convergence across forks and merges
tags: [qa, test, rust]
epic: QA
priority: P0
depends_on: [QA-002, CORE-013]
---

## Description

Simulate two to four peers (user, Copilot fork, remote branch) applying random concurrent operations and merging in random orders. Every peer must reach the same document, with component overrides and text marks intact.

## Acceptance criteria

- [ ] All peers converge to byte-identical JSON exports after merging
- [ ] Runs nightly for 30 minutes with a seed corpus
- [ ] Divergences produce a reproducible seed and fixture
