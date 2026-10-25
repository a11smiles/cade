---
id: DATA-025
title: Test DDL and migrations against real databases in CI
tags: [data, test, rust]
epic: DATA
priority: P1
depends_on: [DATA-013, DATA-014]
---

## Description

CI job spins up PostgreSQL, SQL Server, MySQL and Oracle Free containers and applies schema, migrations and down migrations for a corpus of sample models.

## Acceptance criteria

- [ ] CI applies up and down migrations on all four databases
- [ ] Failures report the model and statement
- [ ] Corpus includes the Claims sample and edge cases
