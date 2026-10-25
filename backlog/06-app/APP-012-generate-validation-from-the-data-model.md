---
id: APP-012
title: Generate validation from the data model
tags: [app, rust, feature]
epic: APP
priority: P0
depends_on: [DATA-020, APP-005]
design: design/exports/v0001-2026-10-25/Code.dc.html
---

## Description

Forms bound to data model entities get field names, types and validation rules (required, min length, date ranges, enums) generated into `validation.ts`. Changing a rule in the data model updates validation and error states.

## Acceptance criteria

- [ ] Binding a form to an entity generates a typed validation module
- [ ] Rules like 'date on or before today' and 'min 20 characters' come from the model
- [ ] Changing a model rule regenerates validation and flags affected screens
- [ ] The generated file is labeled 'from model' and is not hand-edited
