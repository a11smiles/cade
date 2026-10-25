---
id: POLICY-024
title: Let users request a policy exception
tags: [policy, feature, ui, rust]
epic: POLICY
priority: P2
depends_on: [POLICY-020, COPILOT-033]
design: design/exports/v0001-2026-10-25/States.dc.html
---

## Description

From a blocked state, 'Ask admin for an exception' opens a prefilled issue or PR on the org's `.github` repo (configurable in policy) describing the rule, the blocked action and the user's reason, using the user's identity. No document content is included unless the user adds it.

## Acceptance criteria

- [ ] Request opens with rule key, policy version and blocked action prefilled
- [ ] User can edit the reason before submitting
- [ ] The link to the created request appears in the blocked card
- [ ] Requests are recorded in the audit log
