---
id: QA-002
title: Add property-based tests for the document model
tags: [qa, test, rust]
epic: QA
priority: P0
depends_on: [QA-001, CORE-011]
---

## Description

Use `proptest` to generate random sequences of operations (create, move, reparent, set property, bind token, instantiate, detach, undo) and check invariants: the tree stays acyclic, every instance resolves, every token reference resolves or is flagged, bounds caches match a full recompute.

## Acceptance criteria

- [ ] Invariant checks run on at least 10,000 generated sequences in CI
- [ ] Failing cases are shrunk and saved as regression fixtures
- [ ] Tests cover undo/redo interleaved with edits
