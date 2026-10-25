---
id: CORE-023
title: Track ephemeral presence for the Copilot cursor and selections
tags: [core, feature, rust]
epic: CORE
priority: P1
depends_on: [CORE-002]
design: design/exports/v0001-2026-10-25/Main.dc.html
---

## Description

Use an ephemeral store (not persisted) for cursors, selections and "working on" labels so Copilot's live cursor ("Copilot · adding error states") can render on the canvas. Designed so remote collaborators can use the same channel later.

## Acceptance criteria

- [ ] Copilot presence shows a cursor and label on the canvas during a run
- [ ] Presence is never written to the journal or git
- [ ] Presence clears within 2 s of a run ending or crashing
