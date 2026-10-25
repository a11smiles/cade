---
id: HIST-007
title: Restore a past version as a new version
tags: [hist, history, feature, rust, ui]
epic: HIST
priority: P0
depends_on: [HIST-004]
design: design/exports/v0001-2026-10-25/History.dc.html
---

## Description

'Restore this version' writes the old snapshot as the new head and commits it with `CADE-Kind: restore` and the source SHA. Nothing is deleted; later versions remain in the timeline.

## Acceptance criteria

- [ ] Restore creates a new top commit; all later versions remain
- [ ] Restored document matches the selected version exactly
- [ ] Restoring the current version is a no-op with a message
- [ ] Restore is undoable via undo or by restoring the previous head
