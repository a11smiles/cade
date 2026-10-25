---
id: SYS-007
title: Define environments dev, staging and prod
tags: [sys, feature, rust]
epic: SYS
priority: P0
depends_on: [SYS-001]
design: design/exports/v0001-2026-10-25/Deploy.dc.html
---

## Description

Project environments map to subscriptions/resource groups, GitHub environments and param files, with an environment switcher in the System view.

## Acceptance criteria

- [ ] Environments are configurable per project
- [ ] Each environment maps to a GitHub environment name and IaC params
- [ ] Switching environment updates the plan and binding targets
