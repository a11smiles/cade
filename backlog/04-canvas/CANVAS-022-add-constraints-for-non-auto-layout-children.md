---
id: CANVAS-022
title: Add constraints for non-auto-layout children
tags: [canvas, rust, feature]
epic: CANVAS
priority: P0
depends_on: [CANVAS-008]
---

## Description

Children of fixed frames have horizontal and vertical constraints (left, right, left-right, center, scale) that decide how they move when the parent resizes.

## Acceptance criteria

- [ ] Each constraint option behaves as specified when the parent frame is resized
- [ ] Constraints are editable in the properties strip with a visual widget
- [ ] Constraints are ignored (and hidden) for auto layout children
