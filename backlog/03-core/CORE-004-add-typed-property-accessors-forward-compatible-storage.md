---
id: CORE-004
title: Add typed property accessors with forward-compatible storage
tags: [core, feature, rust]
epic: CORE
priority: P0
depends_on: [CORE-003]
---

## Description

Generate typed getters and setters per node kind over the raw property map, with defaults and validation (ranges, enums, units). Unknown properties written by a newer CADE are preserved untouched so older builds never drop data.

## Acceptance criteria

- [ ] Typed accessors exist for every property in the schema doc
- [ ] Invalid values are rejected at the API with a typed error
- [ ] A document with unknown properties round-trips through an older build without loss
- [ ] Accessors are exported to TS for the properties strip
