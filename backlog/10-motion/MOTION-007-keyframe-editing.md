---
id: MOTION-007
title: Implement keyframe add, move, delete and token snapping
tags: [motion, feature, ui]
epic: MOTION
priority: P0
depends_on: [MOTION-005]
design: design/exports/v0001-2026-10-25/Motion.dc.html
---

## Description

Add keys by changing a property with auto-key on, or from the track menu. Drag keys or ranges, multi-select
with marquee, copy/paste, and 'Snap to motion tokens' so durations land on token values.

## Acceptance criteria

- [ ] Auto-key creates a key at the playhead when a property changes
- [ ] Dragging a range with snapping on lands on token durations
- [ ] Copy/paste of keys works across tracks of the same type
- [ ] All edits are undoable
