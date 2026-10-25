---
id: CANVAS-030
title: Create components, variants and instance overrides
tags: [canvas, rust, ts, feature]
epic: CANVAS
priority: P0
depends_on: [CORE-009, CANVAS-028]
---

## Description

Turn a selection into a component, combine components into a variant set with properties (enum, boolean, text, instance swap), and support instance overrides that persist through component updates. Detach and reset overrides are available.

## Acceptance criteria

- [ ] Creating a component and placing instances works; editing the main updates all instances
- [ ] Variant properties appear in the properties strip and switch the instance
- [ ] Overrides on text, fills and visibility survive component edits
- [ ] Reset overrides and Detach instance work and are undoable
