---
id: CANVAS-036
title: Navigate pages
tags: [canvas, ts, rust, feature]
epic: CANVAS
priority: P0
depends_on: [CORE-004]
design: design/exports/v0001-2026-10-25/Main.dc.html
---

## Description

Projects have pages (e.g. Intake flow, Adjuster dashboard, Settings). Add, rename, reorder, duplicate and delete pages; each keeps its own viewport.

## Acceptance criteria

- [ ] Pages can be added, renamed, reordered, duplicated and deleted
- [ ] Switching pages restores that page's last viewport
- [ ] Deleting a page is undoable
