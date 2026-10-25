---
id: CANVAS-023
title: Support responsive breakpoints for one design
tags: [canvas, rust, feature]
epic: CANVAS
priority: P1
depends_on: [CANVAS-021]
design: design/exports/v0001-2026-10-25/Main.dc.html
---

## Description

A frame can define breakpoints (e.g. phone 390, tablet 768, desktop 1280). Property overrides are stored per breakpoint, and the canvas can show the same frame at several breakpoints side by side, replacing duplicate mobile/desktop frames.

## Acceptance criteria

- [ ] Users can add, rename and remove breakpoints on a frame
- [ ] Overrides set at a breakpoint apply only at and above it
- [ ] Side-by-side view shows the frame at each breakpoint and stays in sync
- [ ] Codegen emits matching media queries (APP-008)
