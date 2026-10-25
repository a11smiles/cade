---
id: COPILOT-023
title: Show Before, After and Overlay views for a run
tags: [copilot, feature, ui, rust]
epic: COPILOT
priority: P1
depends_on: [COPILOT-021, CANVAS-035]
design: design/exports/v0001-2026-10-25/Copilot.dc.html
---

## Description

Add a segmented control on the canvas during review: Before (main doc), After (fork), Overlay (After with changed layers outlined in a dashed accent and removed layers ghosted). Rendering both docs uses the same renderer with a doc handle switch.

## Acceptance criteria

- [ ] Switching views re-renders in under 100 ms for a 500-layer frame
- [ ] Overlay outlines exactly the layers in the change set
- [ ] Deleted layers appear ghosted at 30% opacity in Overlay
- [ ] View resets to After when a new run starts
