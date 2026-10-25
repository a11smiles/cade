---
id: DATA-013
title: Parse-check generated SQL per dialect
tags: [data, feature, rust]
epic: DATA
priority: P0
depends_on: [DATA-012]
design: design/exports/v0001-2026-10-25/Data.dc.html
---

## Description

Validate generated SQL offline with a SQL parser for each dialect and show a status chip in the drawer. CI also runs DDL against real database containers.

## Acceptance criteria

- [ ] Status chip shows parse result for the selected dialect and version
- [ ] Parse errors point to the line and the model element
- [ ] CI runs schema and migrations against PostgreSQL, SQL Server, MySQL and Oracle Free containers
