---
id: DATA-014
title: Generate up and down migrations from model changes
tags: [data, feature, rust]
epic: DATA
priority: P0
depends_on: [DATA-012]
design: design/exports/v0001-2026-10-25/Data.dc.html
---

## Description

Diff the model against the last shipped version and generate numbered migrations with up and down scripts, named from the change. Renames are detected and confirmed rather than generated as drop and add.

## Acceptance criteria

- [ ] Each applied change set produces `NNNN_<name>.sql` and `NNNN_down.sql`
- [ ] Renames prompt for confirmation and generate RENAME, not DROP/ADD
- [ ] Down scripts restore the previous schema in tests
- [ ] Migration numbering never collides across branches (timestamp prefix option)
