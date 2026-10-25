---
id: APP-014
title: Add error, empty and loading states as variants
tags: [app, rust, feature]
epic: APP
priority: P1
depends_on: [CANVAS-030]
design: design/exports/v0001-2026-10-25/Copilot.dc.html
---

## Description

Screens can define states (default, error, empty, loading, success) as variants of a frame, so Copilot and users can add error states without duplicating frames.

## Acceptance criteria

- [ ] A frame can have named state variants
- [ ] Switching state on the canvas shows that variant
- [ ] Codegen emits the states as conditional rendering
