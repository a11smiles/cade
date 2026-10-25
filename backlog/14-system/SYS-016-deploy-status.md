---
id: SYS-016
title: Show deploy run status back in CADE
tags: [sys, feature, ui]
epic: SYS
priority: P1
depends_on: [SYS-015, GH-020]
design: design/exports/v0001-2026-10-25/Deploy.dc.html
---

## Description

Poll the workflow run for the deploy PR and show step status, approvals pending and links to logs in the System view.

## Acceptance criteria

- [ ] Run status updates while the view is open
- [ ] Pending approvals are shown with who can approve
- [ ] Failures link to the failing job log
