---
id: SYS-017
title: Detect drift between design and deployed state
tags: [sys, feature, rust]
epic: SYS
priority: P2
depends_on: [SYS-008]
---

## Description

On demand, run what-if for an environment and compare with the design to report drift caused by changes outside CADE.

## Acceptance criteria

- [ ] Drift report lists resources and properties that differ
- [ ] Users can adopt the deployed value into the design or keep the design
- [ ] Drift checks are read-only and use the user's own az session
