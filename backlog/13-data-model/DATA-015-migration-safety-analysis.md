---
id: DATA-015
title: Analyze migrations for reversibility and table rewrites
tags: [data, feature, rust]
epic: DATA
priority: P1
depends_on: [DATA-014]
design: design/exports/v0001-2026-10-25/Data.dc.html
---

## Description

Analyze each migration for data loss, irreversibility, long locks and table rewrites per dialect and version (e.g. constant default on PostgreSQL 11+ is metadata-only). Show findings in the drawer and in Copilot's summary.

## Acceptance criteria

- [ ] Dropping a column or narrowing a type is flagged as data loss
- [ ] Table-rewrite and long-lock operations are flagged with the dialect rule that applies
- [ ] Reversible migrations show a Reversible chip
- [ ] Findings can block Ship by policy
