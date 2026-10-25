---
id: COPILOT-015
title: Attach context with chips and @-mentions
tags: [copilot, feature, ui, ts]
epic: COPILOT
priority: P0
depends_on: [COPILOT-014, CORE-024]
design: design/exports/v0001-2026-10-25/Main.dc.html
---

## Description

Context chips sit above the composer: current selection (auto-added, e.g. 'Selection · Button / Primary'), files (docs/intake.md), the active design system, and other artifacts in the project. Users add context with `@` (layers, files, artifacts, entities, Azure resources) and remove chips with a click. 'Ask Copilot about this' in any properties strip opens the composer with that selection chip.

## Acceptance criteria

- [ ] Selecting a layer updates the selection chip within one frame
- [ ] `@` opens a filtered picker across layers, files, artifacts and data-model entities
- [ ] Removing a chip removes that context from the next request (verified in the request inspector)
- [ ] 'Ask Copilot about this' focuses the composer with the selection chip pre-attached
- [ ] Chips show a token estimate on hover
