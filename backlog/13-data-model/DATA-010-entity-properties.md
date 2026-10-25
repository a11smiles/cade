---
id: DATA-010
title: Edit entity settings in the properties strip
tags: [data, feature, ui]
epic: DATA
priority: P0
depends_on: [DATA-004, DS-011]
design: design/exports/v0001-2026-10-25/Data.dc.html
---

## Description

Selected entity shows table name and schema, key strategy (uuid default, identity, sequence), audit columns toggle, soft delete and classification.

## Acceptance criteria

- [ ] Key strategy options vary by dialect
- [ ] Audit columns add created_at/updated_at (and by) fields
- [ ] Soft delete adds a deleted_at field and partial indexes where supported
