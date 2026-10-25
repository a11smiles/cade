---
id: ARCH-018
title: Parse hand edits in Terraform back into the model
tags: [arch, feature, rust]
epic: ARCH
priority: P2
depends_on: [ARCH-016]
---

## Description

Same as Bicep reverse sync for HCL using an HCL parser. Unknown blocks round-trip as opaque, code-owned blocks.

## Acceptance criteria

- [ ] Property edits in `.tf` files update the canvas
- [ ] New `azurerm_*` resources appear as tiles
- [ ] Unknown blocks survive regeneration unchanged
