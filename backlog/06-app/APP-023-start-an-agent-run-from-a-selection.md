---
id: APP-023
title: Start an agent run from a selection
tags: [app, ts, feature]
epic: APP
priority: P0
depends_on: [APP-022, COPILOT-015]
design: design/exports/v0001-2026-10-25/Main.dc.html
---

## Description

'Ask Copilot about this' in the strip and the chat composer attach the current selection as a context chip (e.g. 'Selection · Button / Primary'), plus files and the design system, so prompts act on that context.

## Acceptance criteria

- [ ] The strip button focuses the composer with the selection chip attached
- [ ] Chips can be removed before sending
- [ ] Changing selection while typing updates the selection chip unless pinned
