---
id: CORE-002
title: Wrap Loro behind the cade-crdt API
tags: [core, feature, rust]
epic: CORE
priority: P0
depends_on: [CORE-001]
---

## Description

`cade-crdt` exposes `Doc`, `Txn`, `Origin` and change subscriptions so no other crate imports Loro directly. Every transaction records an origin: the local user, a Copilot run id, an importer, or a remote merge. This keeps undo grouping, history labels and policy attribution consistent.

## Acceptance criteria

- [ ] No crate other than `cade-crdt` depends on `loro`
- [ ] Every commit carries an `Origin` that is readable from history
- [ ] Subscribers receive batched change events per transaction
- [ ] API is documented with examples and has unit tests for each method
