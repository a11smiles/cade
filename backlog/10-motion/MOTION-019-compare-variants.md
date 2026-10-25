---
id: MOTION-019
title: Compare motion variants side by side
tags: [motion, feature, ui]
epic: MOTION
priority: P2
depends_on: [MOTION-006]
design: design/exports/v0001-2026-10-25/Motion.dc.html
---

## Description

'Compare side by side' plays two timeline variants (before/after or A/B) in sync on a split stage, with
their durations and easings listed. Pick one to keep.

## Acceptance criteria

- [ ] Both variants play in sync from the same playhead
- [ ] Keeping one discards the other in a single undo step
- [ ] Durations and easings of both are listed
