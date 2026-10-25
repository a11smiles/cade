---
id: SYS-011
title: Require approvals for protected environments
tags: [sys, feature, rust, policy]
epic: SYS
priority: P1
depends_on: [SYS-009]
design: design/exports/v0001-2026-10-25/Deploy.dc.html
---

## Description

Configure GitHub environment protection (required reviewers, wait timers, branch rules) for prod through the API when the user has rights, or explain what an admin must set.

## Acceptance criteria

- [ ] Prod environment requires approval by default
- [ ] If the user lacks rights, CADE shows the exact settings for an admin
- [ ] Policy can require approvals for named environments
