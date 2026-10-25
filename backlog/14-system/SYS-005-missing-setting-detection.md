---
id: SYS-005
title: Detect app settings the architecture doesn't define
tags: [sys, feature, rust]
epic: SYS
priority: P0
depends_on: [SYS-002, ARCH-010]
design: design/exports/v0001-2026-10-25/Deploy.dc.html
---

## Description

Scan app code for configuration reads (environment variables, config files) and compare with settings defined on the hosting resource in IaC; report missing or unused settings.

## Acceptance criteria

- [ ] Settings read in code but not defined in IaC are flagged
- [ ] Settings defined but never read are listed as unused
- [ ] Fix adds an app setting or Key Vault reference to IaC
