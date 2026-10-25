---
id: CANVAS-017
title: Place, crop and mask images
tags: [canvas, rust, feature]
epic: CANVAS
priority: P0
depends_on: [CANVAS-003]
---

## Description

Place images by drag-drop, paste or the Image tool. Image fills support fill, fit, crop and tile modes. The Crop tool adjusts the crop rect in place. Any layer can be used as a mask. Images are stored as content-addressed assets in the project.

## Acceptance criteria

- [ ] PNG, JPEG, WebP, GIF (first frame), SVG and HEIC (where the OS supports it) can be placed
- [ ] Fill, fit, crop and tile modes work and are editable on the canvas
- [ ] Crop tool edits the crop rectangle with handles
- [ ] Using a layer as a mask clips the layers above it in the group
- [ ] Duplicate images are stored once by content hash
