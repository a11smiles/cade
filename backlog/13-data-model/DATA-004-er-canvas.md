---
id: DATA-004
title: Render entities as ER cards on the canvas
tags: [data, feature, ui, rust]
epic: DATA
priority: P0
depends_on: [DATA-001, CANVAS-003]
design: design/exports/v0001-2026-10-25/Data.dc.html
---

## Description

Entity cards show name, table, PK/FK markers, type and classification badges. Fields can be added, reordered and edited inline; selecting a card shows the entity properties strip.

## Acceptance criteria

- [ ] Cards render with PK/FK markers and type labels
- [ ] Inline add, rename and reorder of fields works
- [ ] Staged entities from Copilot render with a dashed outline
- [ ] Cards resize to content
