---
id: ARCH-030
title: Apply standard tags and diagnostic settings
tags: [arch, feature, rust]
epic: ARCH
priority: P1
depends_on: [ARCH-013]
---

## Description

Project-level tags and a Log Analytics workspace connection are applied to every resource that supports them, with per-resource overrides.

## Acceptance criteria

- [ ] Project tags appear on all taggable resources in generated IaC
- [ ] Diagnostic settings are generated for resources that support them when a workspace is connected
- [ ] Overrides per resource are possible
