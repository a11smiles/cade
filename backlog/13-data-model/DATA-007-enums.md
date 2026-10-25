---
id: DATA-007
title: Model enums and their values
tags: [data, feature, rust]
epic: DATA
priority: P0
depends_on: [DATA-001]
design: design/exports/v0001-2026-10-25/Data.dc.html
---

## Description

Enums are first-class with ordered values and descriptions; fields can use them. Generation uses native enums where supported and check constraints or lookup tables otherwise.

## Acceptance criteria

- [ ] Enums list in the left panel and are editable
- [ ] Adding a value to an enum generates a safe migration
- [ ] Removing a value warns about data that may use it
- [ ] Lookup-table strategy is selectable per enum
