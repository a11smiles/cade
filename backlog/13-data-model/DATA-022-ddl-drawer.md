---
id: DATA-022
title: Show schema and migration SQL in the drawer
tags: [data, feature, ui]
epic: DATA
priority: P0
depends_on: [DATA-012, DATA-014]
design: design/exports/v0001-2026-10-25/Data.dc.html
---

## Description

Drawer tabs for `schema.sql` and each pending migration with up/down, parse and reversibility chips, synced selection with the canvas.

## Acceptance criteria

- [ ] Selecting an entity scrolls to its CREATE TABLE
- [ ] Pending migration tabs show up and down scripts
- [ ] Chips show parse and reversibility status
