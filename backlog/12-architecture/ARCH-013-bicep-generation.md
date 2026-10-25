---
id: ARCH-013
title: Generate Bicep from the architecture model
tags: [arch, feature, rust]
epic: ARCH
priority: P0
depends_on: [ARCH-001, ARCH-008, ARCH-010]
design: design/exports/v0001-2026-10-25/Architecture.dc.html
---

## Description

Emit idiomatic Bicep: `main.bicep` plus one module per logical group, `existing` references for resources outside the design, parameters for environment-specific values and outputs that bindings need. Output is deterministic so diffs stay small.

## Acceptance criteria

- [ ] Generated Bicep for the Claims Portal sample passes `bicep build` with no errors
- [ ] Regenerating an unchanged model produces byte-identical files
- [ ] Symbolic names are stable across renames of display names
- [ ] Connections generate their implied resources (role assignments, private endpoints, DNS zone groups)
- [ ] Secrets are never written as literal values
