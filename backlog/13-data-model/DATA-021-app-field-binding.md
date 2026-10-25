---
id: DATA-021
title: Bind App form fields to data model fields
tags: [data, feature, rust]
epic: DATA
priority: P0
depends_on: [DATA-020, APP-005]
design: design/exports/v0001-2026-10-25/Deploy.dc.html
---

## Description

App form components can bind to a model field, inheriting label, type, required and validation. Bindings appear in the System bindings table and stay in sync.

## Acceptance criteria

- [ ] Binding a form field shows the model field name on the instance
- [ ] Renaming a model field updates bound App code
- [ ] Deleting a bound field warns and lists affected screens
