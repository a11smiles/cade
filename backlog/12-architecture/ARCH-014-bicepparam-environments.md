---
id: ARCH-014
title: Generate .bicepparam files per environment
tags: [arch, feature, rust]
epic: ARCH
priority: P0
depends_on: [ARCH-013]
design: design/exports/v0001-2026-10-25/Deploy.dc.html
---

## Description

Generate `main.<env>.bicepparam` for dev, staging and prod with environment-specific SKUs, scale and names. The canvas has an environment switcher that shows the effective values for the selected environment.

## Acceptance criteria

- [ ] Each environment gets its own `.bicepparam` file
- [ ] Properties can be set per environment or shared
- [ ] Switching environments on the canvas updates labels and the properties strip
- [ ] Secrets in params use Key Vault references, never literals
