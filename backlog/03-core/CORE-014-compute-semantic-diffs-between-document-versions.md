---
id: CORE-014
title: Compute semantic diffs between document versions
tags: [core, feature, rust]
epic: CORE
priority: P0
depends_on: [CORE-003]
design: design/exports/v0001-2026-10-25/History.dc.html
---

## Description

Given two versions, produce a list of node-level changes (ADD, MOD with property before/after, DEL, MOVE) grouped by frame. Used by staged change sets, version history compare, PR descriptions and review pages.

## Acceptance criteria

- [ ] Diff lists ADD/MOD/DEL/MOVE with node names and paths
- [ ] Token-binding changes show token names, not just resolved values
- [ ] Diff of two 10k-node versions with 200 changes completes under 150 ms
- [ ] Output is stable and deterministic for snapshot tests
