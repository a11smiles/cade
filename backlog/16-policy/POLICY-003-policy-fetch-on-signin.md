---
id: POLICY-003
title: Fetch org policy from the .github repo on sign-in
tags: [policy, feature, rust]
epic: POLICY
priority: P0
depends_on: [POLICY-002, GH-001, FOUND-027]
design: design/exports/v0001-2026-10-25/Onboarding.dc.html
---

## Description

After sign-in, list the user's orgs (and enterprise when available), read `cade/policy.yml` from each org's `.github` repository default branch via the GitHub API, and evaluate `applies_to` team membership. This is the 'Loading Contoso policy' step in onboarding.

## Acceptance criteria

- [ ] Policy is fetched from every org the user belongs to that has `cade/policy.yml`
- [ ] `applies_to: @org/team` applies only to members of that team
- [ ] Users in no org with a policy run with built-in defaults and see 'No organization policy'
- [ ] Fetch completes or times out within 10 s with a retry action
