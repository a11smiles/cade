---
id: ARCH-021
title: Run PSRule for Azure against generated IaC
tags: [arch, feature, rust, security]
epic: ARCH
priority: P1
depends_on: [ARCH-020]
design: design/exports/v0001-2026-10-25/Architecture.dc.html
---

## Description

Run PSRule for Azure rules (Well-Architected baseline) against the expanded templates and show warnings with a fix action that Copilot can stage. Runs without network access.

## Acceptance criteria

- [ ] Warnings list rule id, resource and plain-language explanation
- [ ] Each warning offers a staged fix when one exists
- [ ] Rule baseline is selectable and can be set by policy
- [ ] Results are cached and rerun only for changed resources

## Notes

Requires PowerShell 7 on PATH; detect and explain if missing.
