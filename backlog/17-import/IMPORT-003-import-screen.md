---
id: IMPORT-003
title: Build the Import screen
tags: [import, feature, ui, ts]
epic: IMPORT
priority: P0
depends_on: [IMPORT-001, IMPORT-002, DS-010]
design: design/exports/v0001-2026-10-25/Import.dc.html
---

## Description

Import screen from Start: source list (Spark, Figma, PowerPoint, Visio), found items with checkboxes, 'How
it converts' mapping table, 'Needs attention' warnings from a dry run, primary import action and a success
state with 'Open <workspace>'. Header states that files are converted on this machine.

## Acceptance criteria

- [ ] Selecting a source updates found items, mapping and warnings
- [ ] Warnings come from a dry-run conversion, not static text
- [ ] Import shows progress and a success state that opens the target workspace
- [ ] Screen matches the Import design in dark and light themes and is keyboard accessible
