---
id: DATA-011
title: Model indexes, unique and check constraints
tags: [data, feature, rust]
epic: DATA
priority: P1
depends_on: [DATA-001]
---

## Description

Define indexes (unique, composite, partial where supported) and check constraints. FK columns get an index suggestion automatically.

## Acceptance criteria

- [ ] Indexes and constraints generate correct DDL for all four dialects
- [ ] Missing indexes on FK columns are suggested
- [ ] Unsupported features per dialect are flagged
