---
id: ARCH-002
title: Generate the Azure service catalog from Bicep type schemas
tags: [arch, feature, rust]
epic: ARCH
priority: P0
depends_on: [ARCH-001]
---

## Description

Build a build-time tool in `cade-iac` that reads the published Bicep type index (bicep-types-az) and emits a compact catalog: resource types, API versions, property schemas, enums, required fields and read-only fields. The catalog ships inside the app so property editing works offline.

## Acceptance criteria

- [ ] `cargo xtask gen-azure-catalog` produces a versioned catalog file from a pinned bicep-types-az commit
- [ ] Catalog includes every resource type and its latest stable and preview API versions
- [ ] Property schemas include type, enum values, required, read-only and description
- [ ] Catalog file is under 40 MB compressed and loads in under 300 ms on a mid-range laptop
- [ ] CI fails if the generated catalog differs from the committed one
