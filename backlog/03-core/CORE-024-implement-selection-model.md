---
id: CORE-024
title: Implement the selection model
tags: [core, feature, rust, ui]
epic: CORE
priority: P0
depends_on: [CORE-003]
---

## Description

Selection holds node ids with a primary node, supports multi-select, deep select into groups and instances, and is mirrored to the UI and to Copilot as a context chip. Selection is per window and restored on undo.

## Acceptance criteria

- [ ] Selection changes emit one event per gesture
- [ ] Deep-select into an instance targets the sub-node path used for overrides
- [ ] Selection is included as context when the user asks Copilot
- [ ] Deleted nodes drop from selection automatically
