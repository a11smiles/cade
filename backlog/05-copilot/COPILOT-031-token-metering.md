---
id: COPILOT-031
title: Meter tokens per run, thread, output type and month
tags: [copilot, feature, rust, ui]
epic: COPILOT
priority: P0
depends_on: [COPILOT-002]
design: design/exports/v0001-2026-10-25/Settings.dc.html
---

## Description

Record input, output and cached tokens for every run with model and output type in a local usage store. Show tokens on the run footer ('This run: 186K tokens'), a monthly meter under the composer ('Tokens this month · 4.2M / 10M'), and a breakdown by output type in Settings. Budget limits come from policy (POLICY-021) or the user's Copilot plan when exposed.

## Acceptance criteria

- [ ] Run footer token count equals the sum of usage events for that run
- [ ] Monthly meter resets on the budget's reset date
- [ ] Settings breakdown by output type sums to the monthly total
- [ ] Usage data stays local unless policy enables aggregate reporting (POLICY-022)
