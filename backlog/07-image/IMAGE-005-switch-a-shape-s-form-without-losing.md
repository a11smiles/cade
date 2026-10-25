---
id: IMAGE-005
title: Switch a shape's form without losing its styling
tags: [image, rust, feature]
epic: IMAGE
priority: P1
depends_on: [CANVAS-014, CANVAS-015]
design: design/exports/v0001-2026-10-25/Image.dc.html
---

## Description

The shape properties strip offers Form: circle, rounded square, crescent, star, blob. Switching keeps fill, effects, size and position and swaps the geometry.

## Acceptance criteria

- [ ] Each form option replaces the geometry and keeps styling
- [ ] Switching is one undo step
- [ ] The blob form generates a smooth random shape with a seed that can be re-rolled
