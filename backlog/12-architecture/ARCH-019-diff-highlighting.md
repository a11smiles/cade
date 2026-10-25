---
id: ARCH-019
title: Highlight model and code diffs for staged changes
tags: [arch, feature, ui]
epic: ARCH
priority: P0
depends_on: [ARCH-016]
design: design/exports/v0001-2026-10-25/Architecture.dc.html
---

## Description

When Copilot or the user stages changes, show added lines in green and removed in red in the code drawer, and outline changed resources on the canvas with a dashed accent.

## Acceptance criteria

- [ ] Staged changes show line diffs in the drawer
- [ ] Changed, added and removed resources are visually distinct on the canvas
- [ ] Applying or discarding clears the highlights
- [ ] Diffs are computed against the last applied version
