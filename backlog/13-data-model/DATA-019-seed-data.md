---
id: DATA-019
title: Generate and edit seed data
tags: [data, feature, rust]
epic: DATA
priority: P1
depends_on: [DATA-012]
design: design/exports/v0001-2026-10-25/Prototype.dc.html
---

## Description

Generate realistic sample rows per entity (respecting types, enums and FKs) for prototypes and local testing. PII-classified fields get obviously fake values. Seed data is exported as SQL and JSON.

## Acceptance criteria

- [ ] Seed generation respects FK integrity and enum values
- [ ] PII fields use clearly fake values
- [ ] Prototype mode can use the seed data
- [ ] Users can edit seed rows in a grid
