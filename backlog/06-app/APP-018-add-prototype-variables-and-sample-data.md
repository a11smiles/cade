---
id: APP-018
title: Add prototype variables and sample data
tags: [app, rust, feature]
epic: APP
priority: P1
depends_on: [APP-016, DATA-019]
design: design/exports/v0001-2026-10-25/Prototype.dc.html
---

## Description

Prototype data panel binds to the data model as source, with sample rows (e.g. 12 claims) and validation from model rules. Variables can be set by interactions and read by conditions and text.

## Acceptance criteria

- [ ] Sample rows are generated from the data model and editable
- [ ] Text layers can show variable or sample-row values
- [ ] Conditions evaluate against current variable values in Play
