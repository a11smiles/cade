---
id: CANVAS-035
title: Render staged change overlays for Copilot runs
tags: [canvas, rust, feature]
epic: CANVAS
priority: P0
depends_on: [CORE-013, CANVAS-003]
design: design/exports/v0001-2026-10-25/Copilot.dc.html
---

## Description

Render a run's staged changes on top of the document without committing them: dashed outlines on changed layers, and Before / After / Overlay view modes. Clicking an outlined layer opens its property diff.

## Acceptance criteria

- [ ] Staged changes render without modifying the committed document
- [ ] Before, After and Overlay modes switch instantly
- [ ] Changed layers are outlined with the 'Changed by Copilot' style
- [ ] Clicking an outlined layer shows its property diff with keep/drop actions
