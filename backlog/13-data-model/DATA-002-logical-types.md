---
id: DATA-002
title: Define logical types and per-dialect type mapping
tags: [data, feature, rust]
epic: DATA
priority: P0
depends_on: [DATA-001]
design: design/exports/v0001-2026-10-25/Data.dc.html
---

## Description

Logical types (uuid, text, string(n), int, bigint, decimal(p,s), bool, date, timestamptz, json, bytes, enum) map to physical types for PostgreSQL, SQL Server, MySQL and Oracle, with overrides per field when needed.

## Acceptance criteria

- [ ] Each logical type has a mapping for all four dialects
- [ ] Unsupported combinations (e.g. native enum on SQL Server) map to a documented fallback
- [ ] Per-field physical type override is possible and validated
- [ ] Mapping table is covered by snapshot tests
