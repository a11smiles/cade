---
id: COPILOT-022
title: Review staged changes one by one
tags: [copilot, feature, ui, ts]
epic: COPILOT
priority: P0
depends_on: [COPILOT-021, COPILOT-014]
design: design/exports/v0001-2026-10-25/Copilot.dc.html
---

## Description

Build the 'Staged changes' list and per-change review: clicking a change selects the layer on canvas and shows its property diff; each change has Keep / Drop. Footer actions: Apply N changes, Review one by one, Discard. Text: 'Changes are staged — nothing touches your file until you apply.'

## Acceptance criteria

- [ ] Clicking a change selects and zooms to the layer
- [ ] Dropping a change removes it from the set and updates the count
- [ ] Review one by one steps through changes with keyboard (J/K, Y/N)
- [ ] Apply with some changes dropped applies only the kept ones
