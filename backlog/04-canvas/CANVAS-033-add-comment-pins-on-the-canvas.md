---
id: CANVAS-033
title: Add comment pins on the canvas
tags: [canvas, ts, rust, feature]
epic: CANVAS
priority: P1
depends_on: [CANVAS-006]
design: design/exports/v0001-2026-10-25/Main.dc.html
---

## Description

The Comment tool places a pin anchored to a layer or canvas point. Threads show author avatars, replies and resolve. Comments are stored in the project and synced to PR review comments when shipped (GH epic).

## Acceptance criteria

- [ ] Comment pins stay anchored to their layer as it moves
- [ ] Threads support replies, resolve and reopen
- [ ] Resolved comments can be hidden or shown
- [ ] Comments carry the signed-in GitHub identity
