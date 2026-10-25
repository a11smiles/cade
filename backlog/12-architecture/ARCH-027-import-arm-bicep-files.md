---
id: ARCH-027
title: Import existing ARM JSON or Bicep files
tags: [arch, feature, rust]
epic: ARCH
priority: P1
depends_on: [ARCH-017]
---

## Description

Open an existing ARM template or Bicep file (via `bicep build`) and build a design from it, keeping unrepresentable parts as code-owned blocks.

## Acceptance criteria

- [ ] ARM JSON and Bicep files both import
- [ ] Parameters and outputs are preserved
- [ ] Imported design regenerates code that deploys the same resources
