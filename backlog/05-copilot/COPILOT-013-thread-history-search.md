---
id: COPILOT-013
title: Add thread list, rename, archive and search
tags: [copilot, feature, ui, ts, rust]
epic: COPILOT
priority: P1
depends_on: [COPILOT-012]
design: design/exports/v0001-2026-10-25/Copilot.dc.html
---

## Description

Add a thread switcher to the Copilot panel header: list threads for the current project, rename, archive, start new. Add full-text search across all threads in the project (local index, e.g. Tantivy) with results that jump to the message.

## Acceptance criteria

- [ ] User can create, rename, archive and restore threads from the panel
- [ ] Searching a phrase returns matching messages across threads within 200 ms for 10k messages
- [ ] Clicking a result opens the thread scrolled to the message
- [ ] Archived threads are excluded from search unless 'Include archived' is on
