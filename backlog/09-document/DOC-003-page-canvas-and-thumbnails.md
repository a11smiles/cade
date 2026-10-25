---
id: DOC-003
title: Show pages on the canvas with a pages panel
tags: [doc, feature, ui]
epic: DOC
priority: P0
depends_on: [DOC-002, CANVAS-003]
design: design/exports/v0001-2026-10-25/Document.dc.html
---

## Description

Render pages as sheets on the canvas with zoom and fit-width, and a left 'Pages' panel with live
thumbnails and page numbers. Clicking a thumbnail scrolls to that page. Margins and column guides can be
toggled.

## Acceptance criteria

- [ ] Thumbnails update within 500 ms of edits
- [ ] Zoom levels from 25% to 400% keep text crisp
- [ ] Margin and column guides toggle from the view menu
- [ ] Matches the Document design in dark and light themes
