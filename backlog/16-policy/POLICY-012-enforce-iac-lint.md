---
id: POLICY-012
title: Require IaC lint to pass before an infrastructure PR
tags: [policy, feature, rust]
epic: POLICY
priority: P1
depends_on: [POLICY-002, ARCH-020, ARCH-022]
design: design/exports/v0001-2026-10-25/Architecture.dc.html
---

## Description

With `outputs.iac_lint: required`, Ship IaC to repo runs `bicep build` + `bicep lint` + PSRule for Azure (or `terraform validate` + `tflint`) locally and blocks the PR on errors. Warnings are listed and can be acknowledged if policy allows.

## Acceptance criteria

- [ ] Lint errors block 'Ship IaC to repo'
- [ ] Warnings are shown with rule ids and fix suggestions
- [ ] Lint results are attached to the PR description
- [ ] Missing lint tools prompt a one-click install (preapproved)
