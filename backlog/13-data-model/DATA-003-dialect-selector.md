---
id: DATA-003
title: Switch the target SQL dialect per project
tags: [data, feature, ui]
epic: DATA
priority: P0
depends_on: [DATA-002]
design: design/exports/v0001-2026-10-25/Data.dc.html
---

## Description

Header control selects PostgreSQL, SQL Server, MySQL or Oracle. Field type labels on the canvas and generated DDL update to the selected dialect.

## Acceptance criteria

- [ ] Switching dialect updates labels and DDL without changing the logical model
- [ ] Dialect-specific warnings appear when a feature isn't supported
- [ ] Dialect version (e.g. PostgreSQL 16) is selectable
