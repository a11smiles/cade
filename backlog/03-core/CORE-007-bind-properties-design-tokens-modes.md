---
id: CORE-007
title: Bind properties to design tokens with modes
tags: [core, feature, rust]
epic: CORE
priority: P0
depends_on: [CORE-004]
---

## Description

Any color, number, typography or motion property can hold a token reference instead of a raw value. Tokens live in collections with modes (e.g. dark/light, brand A/B). Resolution walks the reference chain with cycle detection and returns the value for the active mode. Raw values remain allowed unless policy requires tokens.

## Acceptance criteria

- [ ] A property bound to `color.action.primary` updates when the token changes
- [ ] Switching mode re-resolves every bound property
- [ ] Reference cycles are detected and reported, not looped
- [ ] Unresolvable tokens render a visible warning marker and are listed in lint
