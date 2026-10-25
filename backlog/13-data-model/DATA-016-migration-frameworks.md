---
id: DATA-016
title: Export migrations for common frameworks
tags: [data, feature, rust]
epic: DATA
priority: P2
depends_on: [DATA-014]
---

## Description

Optionally export migrations in Flyway, Liquibase, EF Core, Prisma or Alembic layout instead of raw SQL files.

## Acceptance criteria

- [ ] Selected framework layout is generated and validates with the framework's CLI in CI
- [ ] Raw SQL remains the default
- [ ] Switching framework regenerates existing migrations in the new layout without changing their SQL
