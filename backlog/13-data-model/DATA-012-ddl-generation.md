---
id: DATA-012
title: Generate full schema DDL per dialect
tags: [data, feature, rust]
epic: DATA
priority: P0
depends_on: [DATA-002, DATA-007, DATA-011]
design: design/exports/v0001-2026-10-25/Data.dc.html
---

## Description

Generate `schema.sql` creating schemas, types, tables, constraints and indexes in dependency order for the selected dialect. Output is deterministic.

## Acceptance criteria

- [ ] Generated DDL for the sample runs cleanly on each dialect's container in CI
- [ ] Objects are ordered so dependencies come first
- [ ] Regenerating an unchanged model is byte-identical
- [ ] Identifiers are quoted only when needed
