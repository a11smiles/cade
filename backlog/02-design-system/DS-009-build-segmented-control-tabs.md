---
id: DS-009
title: Build Segmented control and Tabs
tags: [ds, feature, ui, a11y]
epic: DS
priority: P0
depends_on: [DS-007]
---

## Description

The mode switch (Design/Prototype/Code), dialect switch, environment switch and filter groups use one SegmentedControl implemented as a radio group. Tabs (Layers/Assets/Tokens, editor file tabs) implement the ARIA tabs pattern with arrow-key navigation.

## Acceptance criteria

- [ ] Segmented control follows the radio-group pattern with arrow keys
- [ ] Tabs follow the ARIA tabs pattern including roving tabindex
- [ ] Both support an optional count badge
