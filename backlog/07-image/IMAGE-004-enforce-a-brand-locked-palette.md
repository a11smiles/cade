---
id: IMAGE-004
title: Enforce a brand-locked palette
tags: [image, rust, feature, policy]
epic: IMAGE
priority: P0
depends_on: [CANVAS-029]
design: design/exports/v0001-2026-10-25/Image.dc.html
---

## Description

When brand lock is on (by project or policy), fills, strokes and effects can only use brand tokens. The fill picker shows only token swatches, the eyedropper snaps to the nearest token, and imported raw colors are flagged.

## Acceptance criteria

- [ ] With brand lock on, the color picker offers only brand tokens
- [ ] Pasted or imported off-brand colors are flagged with a 'nearest token' fix
- [ ] Copilot changes that would break the lock are rejected with a reason
- [ ] The header pill shows 'Brand-locked palette' when active
