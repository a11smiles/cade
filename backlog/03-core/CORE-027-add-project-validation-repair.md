---
id: CORE-027
title: Add project validation and repair
tags: [core, feature, rust]
epic: CORE
priority: P1
depends_on: [CORE-015, CORE-022, CORE-021]
---

## Description

`cade doctor` (CLI and Help menu) checks manifest, schema validity, broken references, missing assets, orphaned journals and token cycles, and offers safe repairs with a backup.

## Acceptance criteria

- [ ] Doctor reports each problem with a location and severity
- [ ] Repairs write a backup first and are undoable
- [ ] Doctor runs in CI against fixture projects
