---
id: CORE-028
title: Ship a headless cade CLI
tags: [core, feature, rust]
epic: CORE
priority: P1
depends_on: [CORE-015, CORE-018]
---

## Description

A `cade` binary for CI and scripting: `init`, `validate`, `doctor`, `export` (PNG/SVG/PDF/pptx/vsdx/Bicep/Terraform/DDL), `merge-driver` and `diff`. Installed alongside the app and available standalone.

## Acceptance criteria

- [ ] `cade export` produces the same output as the app for each format
- [ ] CLI runs headless on Linux CI with no display
- [ ] `cade --help` documents every command
- [ ] Exit codes are documented and stable
