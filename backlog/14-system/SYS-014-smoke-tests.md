---
id: SYS-014
title: Generate smoke tests from prototype flows
tags: [sys, feature, rust]
epic: SYS
priority: P1
depends_on: [SYS-013, APP-015]
design: design/exports/v0001-2026-10-25/Deploy.dc.html
---

## Description

Turn the app's happy-path prototype flow into Playwright smoke tests that run against the slot URL in the workflow.

## Acceptance criteria

- [ ] Happy path flow generates a Playwright test
- [ ] Tests run in the workflow against the slot
- [ ] Failures upload a trace artifact
