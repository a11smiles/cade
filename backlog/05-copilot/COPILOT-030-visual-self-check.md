---
id: COPILOT-030
title: Let the agent render frames and check its own work
tags: [copilot, feature, rust, a11y]
epic: COPILOT
priority: P1
depends_on: [CANVAS-032, COPILOT-019]
design: design/exports/v0001-2026-10-25/Copilot.dc.html
---

## Description

Provide a `render_frames` tool that renders the fork's frames to PNG at 1x and returns images to vision-capable models, plus `check_a11y` that computes text contrast, touch-target sizes and missing labels. Copilot uses these to verify changes before finishing ('Checked contrast · 0 issues').

## Acceptance criteria

- [ ] `render_frames` returns PNGs matching the canvas render (pixel-diff under 1%)
- [ ] `check_a11y` reports contrast ratio per text layer against its background
- [ ] Results appear as a step in the plan
- [ ] Non-vision models receive the a11y report only, not images
