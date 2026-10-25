---
id: CORE-011
title: Route all edits through a validated operations API
tags: [core, feature, rust]
epic: CORE
priority: P0
depends_on: [CORE-003, CORE-004]
---

## Description

Define typed operations (create, delete, move, reparent, set property, bind token, set text, instantiate, detach, group, ungroup) with validation. The UI, Copilot agent tools, importers and scripts all edit through this API, never the CRDT directly, so policy and lint checks run in one place.

## Acceptance criteria

- [ ] Every operation is serializable for agent tool calls and tests
- [ ] Invalid operations are rejected with a typed error and no partial change
- [ ] Batches of operations apply atomically in one transaction
- [ ] Policy hooks (e.g. design-system-only) can veto an operation

## Notes

COPILOT agent tools and POLICY enforcement build on this API.
