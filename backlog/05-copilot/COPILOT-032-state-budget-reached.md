---
id: COPILOT-032
title: Handle the token budget reached state
tags: [copilot, feature, ui]
epic: COPILOT
priority: P0
depends_on: [COPILOT-031, POLICY-021]
design: design/exports/v0001-2026-10-25/States.dc.html
---

## Description

When the budget is used up, show the amber banner ('Your monthly Copilot token budget is used up', usage and reset date), pause Copilot, keep the user's prompt, and offer 'Request more tokens' and 'Save prompt for later'. Manual editing, export, ship and history keep working. Warn at 80% and 95%.

## Acceptance criteria

- [ ] At 100% the composer is disabled and shows 'Copilot paused — budget reached'
- [ ] The unsent prompt is saved and restored after reset
- [ ] 'Request more tokens' sends a request to the admin channel configured in policy
- [ ] Warnings appear once at 80% and 95% per month
- [ ] All non-Copilot features remain usable (smoke test)
