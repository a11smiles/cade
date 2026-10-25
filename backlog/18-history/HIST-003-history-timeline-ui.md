---
id: HIST-003
title: Build the version history timeline with filters
tags: [hist, history, feature, ui, ts]
epic: HIST
priority: P0
depends_on: [HIST-001]
design: design/exports/v0001-2026-10-25/History.dc.html
---

## Description

Timeline panel listing versions newest first with kind tag (COPILOT, YOU, SHIP, IMPORT), time, short SHA, title and detail. Filters: All, Copilot, Mine, Shipped. Opens from the clock button in every workspace header.

## Acceptance criteria

- [ ] Filters update the list instantly
- [ ] Selecting a version highlights it and loads its preview
- [ ] Timeline is virtualized and smooth with 10k versions
- [ ] Keyboard: up/down to move, Enter to compare
