---
id: COPILOT-033
title: Handle a policy-blocked step during a run
tags: [copilot, feature, ui, policy]
epic: COPILOT
priority: P0
depends_on: [POLICY-008, COPILOT-025]
design: design/exports/v0001-2026-10-25/States.dc.html
---

## Description

When a tool call violates policy (e.g. `npm install` with network off, a package not on the allowlist), fail only that step, show the red card with the blocked command and the rule ('tools.network = off'), and offer up to three alternatives: use an approved option (Copilot retries with that constraint), ask admin for an exception (POLICY-024), see the rule.

## Acceptance criteria

- [ ] Blocked step is shown with the exact rule key and policy version
- [ ] Other steps of the run continue
- [ ] 'Use approved alternative' re-runs the step with the constraint added to the prompt
- [ ] Blocked events are written to the audit log (POLICY-023)
