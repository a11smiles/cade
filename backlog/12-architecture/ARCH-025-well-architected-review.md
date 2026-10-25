---
id: ARCH-025
title: Review the design against Azure Well-Architected
tags: [arch, feature, rust]
epic: ARCH
priority: P1
depends_on: [ARCH-021]
design: design/exports/v0001-2026-10-25/Architecture.dc.html
---

## Description

Copilot reviews the design across reliability, security, cost, operational excellence and performance, combining PSRule results with design-level checks (single points of failure, missing backups, no zone redundancy). Findings become stageable fixes.

## Acceptance criteria

- [ ] Review produces findings grouped by pillar with severity
- [ ] Each finding links to the resource and offers a staged fix where possible
- [ ] Review runs on demand and before Ship if policy requires
- [ ] Findings can be dismissed with a reason that is saved in the project
