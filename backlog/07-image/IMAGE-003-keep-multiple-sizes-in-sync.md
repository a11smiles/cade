---
id: IMAGE-003
title: Keep multiple sizes in sync
tags: [image, rust, feature]
epic: IMAGE
priority: P0
depends_on: [IMAGE-001, CANVAS-022]
design: design/exports/v0001-2026-10-25/Image.dc.html
---

## Description

All sizes share one layer set. Each size stores layout overrides (position, scale, visibility, crop) while content and styles stay shared, so editing the sun in one size updates all four. Show 'Updated in all N' after a shared edit.

## Acceptance criteria

- [ ] Editing a shared property updates every size
- [ ] Position and visibility can be overridden per size
- [ ] A per-size override can be reset to the shared value
- [ ] The canvas shows a confirmation like 'Sun updated in all 4'
