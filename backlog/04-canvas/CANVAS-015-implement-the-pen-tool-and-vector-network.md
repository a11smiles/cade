---
id: CANVAS-015
title: Implement the pen tool and vector network editing
tags: [canvas, rust, feature]
epic: CANVAS
priority: P0
depends_on: [CANVAS-014]
---

## Description

Create and edit vector networks: place points, drag bezier handles, close paths, add/remove points, convert point types (mirrored, asymmetric, disconnected), and bend segments. Double-click a path to enter vector edit mode.

## Acceptance criteria

- [ ] Pen tool creates open and closed paths with curves
- [ ] Vector edit mode lets users move points and handles and change point types
- [ ] Points can be added on a segment and deleted without breaking the path
- [ ] A vector network with branching points (more than two segments at a point) is supported
