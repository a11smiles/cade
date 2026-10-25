---
id: DATA-018
title: Import a model from an existing DDL file
tags: [data, feature, rust]
epic: DATA
priority: P1
depends_on: [DATA-013]
---

## Description

Parse an existing `schema.sql` into the model, keeping unsupported statements as raw SQL blocks.

## Acceptance criteria

- [ ] DDL for each dialect imports to entities and relationships
- [ ] Unsupported statements are preserved and listed
- [ ] Regenerating DDL from the imported model produces an equivalent schema in the CI database containers
