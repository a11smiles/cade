---
id: ARCH-026
title: Reverse-engineer a design from an existing resource group
tags: [arch, feature, rust]
epic: ARCH
priority: P1
depends_on: [ARCH-023, ARCH-007]
---

## Description

Import resources from a live resource group (read via Azure Resource Graph and ARM export) into the model, infer containment and connections (VNet integration, private endpoints, role assignments) and lay them out automatically.

## Acceptance criteria

- [ ] Importing the sample resource group produces tiles for every supported resource
- [ ] Unsupported resources are listed and imported as generic tiles
- [ ] Inferred connections are marked as inferred until confirmed
- [ ] Import is read-only against Azure
