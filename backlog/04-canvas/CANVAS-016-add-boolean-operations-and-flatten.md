---
id: CANVAS-016
title: Add boolean operations and flatten
tags: [canvas, rust, feature]
epic: CANVAS
priority: P1
depends_on: [CANVAS-015]
---

## Description

Union, subtract, intersect and exclude for two or more shapes, kept as a live boolean group that can be edited, plus Flatten to bake it into a single path. Use a robust path boolean library with tolerance handling.

## Acceptance criteria

- [ ] All four operations produce correct outlines on overlapping, nested and self-intersecting shapes
- [ ] Boolean groups stay editable; moving a child updates the result live
- [ ] Flatten produces one path with the same rendered outline
- [ ] Operations on 200+ path segments finish in under 50 ms
