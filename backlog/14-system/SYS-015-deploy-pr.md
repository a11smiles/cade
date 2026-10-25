---
id: SYS-015
title: Open the deploy PR from the System view
tags: [sys, feature, rust]
epic: SYS
priority: P0
depends_on: [SYS-009, GH-010]
design: design/exports/v0001-2026-10-25/Deploy.dc.html
---

## Description

Open Deploy PR writes the workflow and any IaC/migration changes to a branch and opens a PR, with a description listing the plan. CADE never deploys directly.

## Acceptance criteria

- [ ] Button is disabled with a reason while blockers exist
- [ ] PR description lists plan steps and environment
- [ ] No deployment happens from the desktop app
