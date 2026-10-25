---
id: DECK-010
title: Add a spreadsheet-style chart data editor with paste and CSV
tags: [deck, feature, ui, ts]
epic: DECK
priority: P0
depends_on: [DECK-007]
design: design/exports/v0001-2026-10-25/Slides.dc.html
---

## Description

'Edit values' opens a grid editor for categories and series: add/remove rows and columns, rename series,
paste tab-separated data from Excel or Sheets, and import CSV or XLSX. Placeholder cells are highlighted and
counted. 'Attach data' from Copilot uses the same import path.

## Acceptance criteria

- [ ] Pasting a 10×4 range from Excel fills the grid and updates the chart in one undo step
- [ ] CSV and XLSX import detect header row and number formats
- [ ] Placeholder cells are highlighted and the count is shown in the editor footer
- [ ] Invalid values (text in numeric series) are flagged without blocking other edits
- [ ] Editor is fully keyboard operable (arrow keys, Enter, Tab)
