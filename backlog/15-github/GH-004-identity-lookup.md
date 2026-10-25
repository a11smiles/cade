---
id: GH-004
title: Look up the user's identity, orgs and Copilot seat
tags: [gh, feature, rust]
epic: GH
priority: P0
depends_on: [GH-001]
design: design/exports/v0001-2026-10-25/Onboarding.dc.html
---

## Description

Fetch the user's login, name, avatar, org memberships and Copilot access to show on first run and to pick the org whose policy applies.

## Acceptance criteria

- [ ] First run shows the signed-in login and enterprise
- [ ] Copilot access is checked and a clear message shown when missing
- [ ] Users in several orgs can pick which org's policy applies
