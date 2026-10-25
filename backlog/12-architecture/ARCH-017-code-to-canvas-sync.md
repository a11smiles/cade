---
id: ARCH-017
title: Parse hand edits in Bicep back into the model
tags: [arch, feature, rust]
epic: ARCH
priority: P1
depends_on: [ARCH-016]
design: design/exports/v0001-2026-10-25/Architecture.dc.html
---

## Description

Allow editing the generated Bicep directly. Parse it with the Bicep compiler's JSON output (via `bicep build` to ARM JSON) and reconcile changes into the model; constructs CADE can't represent are kept as opaque blocks that round-trip untouched.

## Acceptance criteria

- [ ] Changing a property value in code updates the canvas within one second
- [ ] Adding a resource in code adds a tile in the right container
- [ ] Unrepresentable constructs (loops, complex expressions) are preserved verbatim and marked as code-owned
- [ ] Conflicting edits show a merge prompt instead of overwriting
