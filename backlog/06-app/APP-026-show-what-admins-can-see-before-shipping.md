---
id: APP-026
title: Show what admins can see before shipping
tags: [app, ts, ui, feature, policy]
epic: APP
priority: P0
depends_on: [APP-024, POLICY-022]
design: design/exports/v0001-2026-10-25/Ship.dc.html
---

## Description

The Ship screen lists what admins can and can't see (app exists and repo, dependencies and scan status vs. designs, drafts, prompts, Copilot conversations), generated from the active policy.

## Acceptance criteria

- [ ] The visibility list is generated from policy, not hardcoded
- [ ] Changing policy changes the list
- [ ] Copy matches the design reference
