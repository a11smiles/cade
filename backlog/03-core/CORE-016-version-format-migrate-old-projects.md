---
id: CORE-016
title: Version the format and migrate old projects
tags: [core, feature, rust]
epic: CORE
priority: P0
depends_on: [CORE-015]
---

## Description

Migrations run on open from any older format version, write a backup first, and are covered by fixture tests. Opening a project from a newer format version opens read-only with an upgrade prompt rather than risking data loss.

## Acceptance criteria

- [ ] Fixture projects for each format version upgrade cleanly in tests
- [ ] A backup is written before any migration
- [ ] Newer-format projects open read-only with a clear message
- [ ] Migrations are idempotent
