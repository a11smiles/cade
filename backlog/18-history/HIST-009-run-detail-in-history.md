---
id: HIST-009
title: Show Copilot run details in history
tags: [hist, history, feature, ui]
epic: HIST
priority: P1
depends_on: [HIST-001, COPILOT-012]
design: design/exports/v0001-2026-10-25/History.dc.html
---

## Description

For COPILOT versions, show the prompt, model and token count from the run (from trailers plus the thread when available), with a link to open the thread at that run.

## Acceptance criteria

- [ ] Copilot versions show prompt, model and tokens
- [ ] If the thread file is missing, trailers still show model and tokens
- [ ] 'Open thread' jumps to the run in the Copilot panel
- [ ] Non-Copilot versions don't show the run card
