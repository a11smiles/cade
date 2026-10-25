---
id: CORE-001
title: "Spike: validate Loro for a 10k-layer design document"
tags: [core, spike, rust, perf]
epic: CORE
priority: P0
depends_on: [FOUND-001]
---

## Description

Before committing the schema, benchmark Loro with a synthetic document of 10,000 nodes in a MovableTree with 20 properties each: load time, snapshot size, update size for a property change, tree move cost, undo manager behavior and fork/merge cost. Record results and any API gaps in an ADR.

## Acceptance criteria

- [ ] Benchmarks run with `cargo bench -p cade-crdt`
- [ ] ADR `docs/adr/0002-crdt-loro.md` records numbers and decision
- [ ] Cold load of the 10k document is under 300 ms on the reference machine or the ADR names a mitigation
- [ ] Fork, edit and merge of 500 changes is measured
