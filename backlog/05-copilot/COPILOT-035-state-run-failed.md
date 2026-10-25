---
id: COPILOT-035
title: Recover from a run that fails mid-way
tags: [copilot, feature, ui, rust]
epic: COPILOT
priority: P0
depends_on: [COPILOT-021, COPILOT-025]
design: design/exports/v0001-2026-10-25/States.dc.html
---

## Description

If a tool times out or errors fatally, stop the run, keep completed steps staged, and show 'Run stopped at step N of M' with the failing command. Offer Retry from step N (optionally with a longer timeout), Keep completed steps, Undo the whole run.

## Acceptance criteria

- [ ] Failed run keeps steps 1..N-1 staged and reviewable
- [ ] Retry from step N continues on the same fork
- [ ] Keep completed steps applies only those
- [ ] Undo the whole run leaves the document unchanged
