---
id: ARCH-007
title: Model subscription, resource group, VNet and subnet containment
tags: [arch, feature, rust]
epic: ARCH
priority: P0
depends_on: [ARCH-001]
design: design/exports/v0001-2026-10-25/Architecture.dc.html
---

## Description

Implement containers as first-class nodes with rules about what may live where: resource groups in subscriptions, VNets in resource groups, subnets in VNets, and only subnet-capable resources (private endpoints, delegated services) inside subnets. Containers draw as dashed regions with their name and address space.

## Acceptance criteria

- [ ] Dropping a resource onto a container re-parents it and updates its scope in generated IaC
- [ ] Invalid placements are refused with a reason ("Key Vault can't be placed in a subnet; add a private endpoint instead")
- [ ] Subnet address ranges are validated against the VNet range and against each other
- [ ] Containers auto-grow to fit children and can be resized manually
- [ ] Multiple subscriptions and resource groups can coexist on one canvas
