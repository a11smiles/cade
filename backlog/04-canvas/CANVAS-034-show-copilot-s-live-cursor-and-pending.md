---
id: CANVAS-034
title: Show Copilot's live cursor and pending regions
tags: [canvas, rust, ts, feature]
epic: CANVAS
priority: P0
depends_on: [CORE-023, CANVAS-003]
design: design/exports/v0001-2026-10-25/Main.dc.html
---

## Description

When Copilot edits the canvas, show its labeled cursor (e.g. 'Copilot · adding error states') and dashed violet placeholders for frames being generated, so users can see where the agent is working without blocking manual edits elsewhere.

## Acceptance criteria

- [ ] Copilot cursor appears at the layer the agent is editing with a status label
- [ ] Frames being generated show a dashed placeholder with a skeleton fill
- [ ] Users can keep editing other layers while the agent works
- [ ] Cursor and placeholders disappear when the run finishes or is stopped

## Notes

Absorbs COPILOT-026, which duplicated this item and was removed during backlog reconciliation.
