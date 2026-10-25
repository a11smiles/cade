---
id: DATA-017
title: Reverse-engineer a model from a live database
tags: [data, feature, rust, security]
epic: DATA
priority: P1
depends_on: [DATA-001, DATA-002]
---

## Description

Connect to a database, read its catalog and build entities, fields, relationships, enums and indexes. The connection string lives only in memory for the session or in the OS keychain if the user chooses; it is never written to the project or logs.

## Acceptance criteria

- [ ] Supports PostgreSQL, SQL Server, MySQL and Oracle
- [ ] Connection strings are never written to disk outside the OS keychain
- [ ] Read-only queries only; no DDL is executed
- [ ] Imported model lays out automatically
- [ ] Agent network policy is respected (user-initiated only when off)
