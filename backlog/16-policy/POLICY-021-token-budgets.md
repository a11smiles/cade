---
id: POLICY-021
title: Enforce token budgets per user and cost center
tags: [policy, feature, rust]
epic: POLICY
priority: P1
depends_on: [POLICY-001, COPILOT-031]
design: design/exports/v0001-2026-10-25/Settings.dc.html
---

## Description

`budgets` sets monthly token limits per user, per team or per cost center (mapped from team membership), with a reset day. CADE enforces the budget client-side using its local meter and shows it in Settings ('4.2M of 10M tokens · monthly budget set by Contoso'). Where GitHub exposes server-side Copilot usage or limits, those take precedence.

## Acceptance criteria

- [ ] Budget for a user resolves from the most specific match (user > team > cost center > default)
- [ ] Reaching the budget triggers COPILOT-032
- [ ] Budget and reset date appear in Settings and the composer meter
- [ ] Server-side limits, when available, override the client estimate

## Notes

Client-side enforcement can be bypassed by a determined user; document it as a guardrail, with server-side Copilot billing controls as the hard limit.
