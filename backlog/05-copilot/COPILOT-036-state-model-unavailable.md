---
id: COPILOT-036
title: Fall back when a model is unavailable
tags: [copilot, feature, ui]
epic: COPILOT
priority: P1
depends_on: [COPILOT-011]
design: design/exports/v0001-2026-10-25/States.dc.html
---

## Description

If the chosen model is unavailable, run on Auto (or the next allowed model) and mark the answer 'Answered with a different model' with Keep / Retry with original. Never silently switch to a model the policy disallows.

## Acceptance criteria

- [ ] Fallback only picks models allowed by policy
- [ ] The run records both requested and actual model
- [ ] 'Retry with Opus' re-runs when the model is available again
- [ ] If no allowed model is available, show the runtime-unavailable state instead
