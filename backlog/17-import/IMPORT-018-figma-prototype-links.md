---
id: IMPORT-018
title: Convert Figma prototype connections to prototype flows
tags: [import, feature, rust]
epic: IMPORT
priority: P1
depends_on: [IMPORT-014, APP-016]
design: design/exports/v0001-2026-10-25/Import.dc.html
---

## Description

Map prototype interactions (on click/tap, hover, after delay) and actions (navigate, overlay, back, scroll
to) with transitions to CADE prototype flows and interactions. Flow starting points become flows.

## Acceptance criteria

- [ ] Starting points become named flows
- [ ] Navigate and overlay actions play back in the prototype player
- [ ] Unsupported interactions (e.g. variables-driven) are reported
