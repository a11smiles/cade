---
id: IMAGE-007
title: Edit opacity and layer order for illustration work
tags: [image, rust, feature]
epic: IMAGE
priority: P0
depends_on: [CANVAS-020, CANVAS-024]
design: design/exports/v0001-2026-10-25/Image.dc.html
---

## Description

Opacity field, blend mode and quick reorder in the Layers list (background, hills, sun, text, button, logo) tuned for illustration, with lock-background by default.

## Acceptance criteria

- [ ] Opacity edits render live and commit one undo step
- [ ] Background layer is locked by default in new Image projects
- [ ] Reordering in the list updates the canvas immediately
