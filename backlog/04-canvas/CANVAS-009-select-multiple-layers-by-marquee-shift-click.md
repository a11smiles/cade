---
id: CANVAS-009
title: Select multiple layers by marquee, shift-click and select-all
tags: [canvas, rust, feature]
epic: CANVAS
priority: P0
depends_on: [CANVAS-006]
---

## Description

Support marquee selection (touch vs contain with Alt), Shift-click add/remove, Select All within the current frame, Select Same (fill, stroke, component, font). Multi-selection shows a combined bounding box and mixed values in the properties strip.

## Acceptance criteria

- [ ] Marquee selects layers that intersect; Alt-marquee selects only fully contained layers
- [ ] Shift-click toggles membership in the selection
- [ ] Select Same fill/stroke/component/font selects matching layers in the current page
- [ ] Properties strip shows 'Mixed' for differing values and edits apply to all
