---
id: DECK-007
title: Define a native chart object with series and data table
tags: [deck, feature, rust]
epic: DECK
priority: P0
depends_on: [CORE-003]
design: design/exports/v0001-2026-10-25/Slides.dc.html
---

## Description

A `Chart` layer type stores chart kind (bar, column, line, area, stacked bar, donut, scatter), a data
table (categories × series), per-series style bound to color tokens, axis settings, label placement and
legend settings. The same chart type is used in Deck, Document and App so charts can be moved between
outputs. Values may be placeholders (e.g. [A]) that are tracked as unfilled.

## Acceptance criteria

- [ ] Chart data, kind and series styles persist in the document and survive concurrent edits
- [ ] Placeholder values are stored as typed placeholders and counted by the export and print checks
- [ ] Switching chart kind preserves data and series token bindings
- [ ] Charts serialize to JSON export for review diffs
