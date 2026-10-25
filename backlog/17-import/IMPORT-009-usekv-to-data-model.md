---
id: IMPORT-009
title: Convert useKV data into Data model entities and seed data
tags: [import, feature, rust]
epic: IMPORT
priority: P0
depends_on: [IMPORT-006, DATA-001]
design: design/exports/v0001-2026-10-25/Import.dc.html
---

## Description

Find `useKV` keys and their TypeScript types, infer entities and fields, and create Data model entities
with seed data from default values. Replace `useKV` calls with a generated data-access layer bound to the
model.

## Acceptance criteria

- [ ] Each useKV key with an object or array type becomes an entity
- [ ] Field types are inferred from TypeScript types
- [ ] Default values become seed data
- [ ] The generated data layer compiles and the app runs
