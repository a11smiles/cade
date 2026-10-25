---
id: DOC-007
title: Add the stat row block with placeholder detection
tags: [doc, feature, rust]
epic: DOC
priority: P1
depends_on: [DOC-004]
design: design/exports/v0001-2026-10-25/Document.dc.html
---

## Description

A row of 2–4 statistics (value + caption). Values like [X] are typed placeholders and counted by the
print check until filled. Values can bind to a data source cell or chart value so they update together.

## Acceptance criteria

- [ ] Stat rows support 2, 3 or 4 columns and wrap on narrow pages
- [ ] Placeholder values are counted in the print check
- [ ] A stat can bind to a chart value and updates when the chart data changes
