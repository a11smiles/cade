---
id: POLICY-018
title: Build the admin console shell and overview
tags: [policy, feature, ui]
epic: POLICY
priority: P1
depends_on: [POLICY-017, GH-001]
design: design/exports/v0001-2026-10-25/Admin.dc.html
---

## Description

Admins (org owners or members of the policy repo's CODEOWNERS) see Settings › Org policy (admin) with sections Overview, Policy, Design systems, Templates, Cloud targets, Usage & adoption. Overview shows active installs, projects shipped, % on current policy version and policy version, from aggregate reporting only.

## Acceptance criteria

- [ ] Admin entry appears only for users with write access to the org's `.github` repo
- [ ] Overview cards show '—' with an explanation when reporting is disabled
- [ ] Sections are reachable by keyboard and deep-linkable
- [ ] Non-admins cannot open admin routes
