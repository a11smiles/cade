---
id: DATA-001
title: Define the entity, field and relationship model
tags: [data, feature, rust]
epic: DATA
priority: P0
depends_on: [CORE-003]
design: design/exports/v0001-2026-10-25/Data.dc.html
---

## Description

Add the data model document type in `cade-ddl`: entities (with schema and table name), fields (logical type, nullability, default, constraints), enums, relationships (cardinality, on-delete behavior) and indexes, stored in the shared Loro document.

## Acceptance criteria

- [ ] Entities, fields, enums, relationships and indexes round-trip through a snapshot
- [ ] Concurrent edits to different fields of one entity merge cleanly
- [ ] Each element has a stable id used by App bindings
- [ ] Unit tests cover rename, reorder and delete
