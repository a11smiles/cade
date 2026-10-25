---
id: ARCH-020
title: Run bicep build and the Bicep linter locally
tags: [arch, feature, rust]
epic: ARCH
priority: P0
depends_on: [ARCH-013]
design: design/exports/v0001-2026-10-25/Architecture.dc.html
---

## Description

Run the Bicep CLI (bundled or found on PATH) to build and lint generated files, mapping diagnostics back to resources on the canvas. Runs on save and before Ship.

## Acceptance criteria

- [ ] Diagnostics appear as badges on the affected resource tiles
- [ ] Clicking a diagnostic selects the resource and its code line
- [ ] Missing Bicep CLI shows an install prompt, not an error dump
- [ ] Lint rules can be configured through `bicepconfig.json` and policy
