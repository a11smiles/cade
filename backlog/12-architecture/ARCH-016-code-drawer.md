---
id: ARCH-016
title: Show generated IaC in a synced code drawer
tags: [arch, feature, ui, ts]
epic: ARCH
priority: P0
depends_on: [ARCH-013, ARCH-015]
design: design/exports/v0001-2026-10-25/Architecture.dc.html
---

## Description

Bottom drawer with tabs for each generated file, syntax highlighting, line numbers and the selected resource's block highlighted. Selecting code selects the resource on the canvas and vice versa.

## Acceptance criteria

- [ ] Selecting a resource scrolls to and highlights its block
- [ ] Placing the cursor in a block selects that resource on the canvas
- [ ] Status chips show build, lint and sync state
- [ ] Drawer height and open state persist per project
