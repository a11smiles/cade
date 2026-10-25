---
id: IMPORT-008
title: Map Tailwind and inline styles to design-system tokens
tags: [import, feature, rust]
epic: IMPORT
priority: P0
depends_on: [IMPORT-007, CORE-007]
design: design/exports/v0001-2026-10-25/Import.dc.html
---

## Description

Resolve Tailwind classes and inline styles to values, then map each to the nearest token: colors by
OKLab distance, spacing and radius to the nearest scale step, type to the nearest text style. Values beyond a
threshold are kept raw and reported ('2 colors aren't in Contoso DS') with Copilot suggestions.

## Acceptance criteria

- [ ] Colors within ΔE 2 map silently to tokens
- [ ] Off-palette colors are reported with the nearest token suggestion
- [ ] Spacing and radius snap to scale steps
- [ ] Mapping thresholds are configurable by policy
