---
id: COPILOT-025
title: Show the agent plan and allow stopping a run
tags: [copilot, feature, ui, ts]
epic: COPILOT
priority: P0
depends_on: [COPILOT-002, COPILOT-014]
design: design/exports/v0001-2026-10-25/Copilot.dc.html
---

## Description

Render plan updates as a checklist (done, in progress, pending) inside the run card and the wide view. 'Stop run' cancels at the next safe point, keeps completed steps staged, and labels the run as stopped. Show run metadata: model, tokens so far, elapsed time.

## Acceptance criteria

- [ ] Plan steps update live as events arrive
- [ ] Stop run ends tool execution within 1 s and keeps completed staged changes
- [ ] Run card shows model, token count and elapsed time
- [ ] A stopped run can be resumed with 'Continue' which starts a new run seeded with the remaining plan
