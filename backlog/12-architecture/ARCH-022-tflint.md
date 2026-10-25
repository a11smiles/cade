---
id: ARCH-022
title: Run terraform validate and tflint for Terraform projects
tags: [arch, feature, rust]
epic: ARCH
priority: P1
depends_on: [ARCH-015]
---

## Description

For Terraform projects run `terraform validate` and tflint with the azurerm ruleset, surfacing diagnostics on resources.

## Acceptance criteria

- [ ] Diagnostics map back to resources
- [ ] Missing tools show an install prompt
- [ ] tflint config can be set by policy
