---
id: CORE-012
title: Implement undo and redo grouped by origin
tags: [core, feature, rust]
epic: CORE
priority: P0
depends_on: [CORE-002, CORE-011]
design: design/exports/v0001-2026-10-25/Copilot.dc.html
---

## Description

Use Loro's undo manager scoped to the local user's origin so undo never reverts a merged remote change. A whole applied Copilot run is one undo group ("Undo restores the whole run"). Gesture edits (drag, resize) coalesce into one step. Selection is restored with each undo step.

## Acceptance criteria

- [ ] Undo after applying a Copilot run reverts every change in that run in one step
- [ ] Dragging a layer creates one undo step regardless of pointer events
- [ ] Undo does not revert changes from a merged branch
- [ ] Undo history survives reopening the project for the current session journal
