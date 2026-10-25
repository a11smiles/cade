---
id: DATA-005
title: Draw relationships with cardinality markers
tags: [data, feature, ui]
epic: DATA
priority: P0
depends_on: [DATA-004]
design: design/exports/v0001-2026-10-25/Data.dc.html
---

## Description

Create a relationship by dragging between fields; choose 1–1, 1–many, many–many (generates a junction table) and on-delete behavior. Lines show cardinality marks at both ends.

## Acceptance criteria

- [ ] Dragging from a field to an entity creates a foreign key
- [ ] Many-to-many creates a junction entity automatically
- [ ] On-delete options are cascade, restrict, set null
- [ ] Cardinality marks render at both ends
