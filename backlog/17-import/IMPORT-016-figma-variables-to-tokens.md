---
id: IMPORT-016
title: Convert Figma variables and modes to tokens
tags: [import, feature, rust]
epic: IMPORT
priority: P0
depends_on: [IMPORT-013, CORE-007]
design: design/exports/v0001-2026-10-25/Import.dc.html
---

## Description

Variables and collections become tokens; modes (e.g. light and dark) become token modes; aliases are
preserved. If the variables endpoint is not available on the user's Figma plan, fall back to published
styles and report it.

## Acceptance criteria

- [ ] Variable collections map to token groups with mode values
- [ ] Aliased variables stay aliases
- [ ] Layers bound to variables stay bound to the resulting tokens
- [ ] Fallback to styles is reported when variables are unavailable
