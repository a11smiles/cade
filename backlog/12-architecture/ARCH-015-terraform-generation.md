---
id: ARCH-015
title: Generate Terraform (azurerm) from the architecture model
tags: [arch, feature, rust]
epic: ARCH
priority: P0
depends_on: [ARCH-001, ARCH-008]
design: design/exports/v0001-2026-10-25/Architecture.dc.html
---

## Description

Emit Terraform using the azurerm provider (and azapi for resources azurerm lacks), with a module per logical group, `variables.tf`, `outputs.tf`, per-environment `.tfvars` and a backend block placeholder. Users pick Bicep or Terraform per project.

## Acceptance criteria

- [ ] Generated Terraform for the sample passes `terraform validate`
- [ ] Resources without azurerm support fall back to `azapi_resource` with a comment
- [ ] Provider versions are pinned in `versions.tf`
- [ ] Output is deterministic across regenerations
- [ ] Switching the project from Bicep to Terraform keeps the model unchanged
