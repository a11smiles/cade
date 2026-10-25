---
id: HIST-006
title: List what changed between two versions
tags: [hist, history, feature, rust]
epic: HIST
priority: P1
depends_on: [HIST-004, CORE-014]
design: design/exports/v0001-2026-10-25/History.dc.html
---

## Description

Compute a layer-level diff between two documents (ADD/MOD/DEL with property names) for the 'What changed since then' panel, reusing the change-set diff format from COPILOT-021.

## Acceptance criteria

- [ ] Diff lists added, modified and deleted layers with changed properties
- [ ] Diff between identical versions is empty
- [ ] Clicking a diff row selects the layer in the compare view
- [ ] Diff of 5k-layer documents completes in under 2 s
