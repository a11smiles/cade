---
id: ARCH-029
title: Respect landing-zone and Azure Policy constraints
tags: [arch, feature, rust, policy]
epic: ARCH
priority: P1
depends_on: [ARCH-023]
design: design/exports/v0001-2026-10-25/Architecture.dc.html
---

## Description

Read Azure Policy assignments for the target scope (with the user's session) and the org's CADE policy, and warn when the design violates allowed regions, SKUs, required tags or deny effects before deploy.

## Acceptance criteria

- [ ] Allowed-locations and allowed-SKU policies flag violating resources
- [ ] Required tags are added to generated IaC automatically
- [ ] Deny-effect violations block Ship with an explanation
- [ ] Works offline with the last cached policy set and says so
