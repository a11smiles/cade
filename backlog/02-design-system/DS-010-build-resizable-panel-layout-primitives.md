---
id: DS-010
title: Build resizable panel layout primitives
tags: [ds, feature, ui]
epic: DS
priority: P0
depends_on: [DS-005]
design: design/exports/v0001-2026-10-25/Main.dc.html
---

## Description

App frame with header, tool rail, left panel, canvas region, right Copilot column and bottom drawer (code/DDL/timeline). Panels resize with drag handles and keyboard, collapse, and remember sizes per workspace. The canvas region reports its rect to Rust for the native surface.

## Acceptance criteria

- [ ] Panels resize by mouse and keyboard and persist per workspace
- [ ] Canvas rect updates are sent to Rust on every layout change
- [ ] Minimum widths keep panels usable at 1280×800
- [ ] Drawer toggles without the canvas reflowing content
