---
id: HIST-005
title: Compare a past version with the current one side by side
tags: [hist, history, feature, ui, rust]
epic: HIST
priority: P0
depends_on: [HIST-004, CANVAS-003]
design: design/exports/v0001-2026-10-25/History.dc.html
---

## Description

Render the selected version and current version of the same frame side by side, with a frame picker and synced pan/zoom. Optional onion-skin overlay mode highlights changed regions.

## Acceptance criteria

- [ ] Both panes render the same frame at the same zoom
- [ ] Pan/zoom is synchronized
- [ ] Overlay mode highlights pixels that differ
- [ ] Selecting the current version shows a single pane
