---
id: ARCH-028
title: Lay out architecture diagrams automatically
tags: [arch, feature, rust]
epic: ARCH
priority: P1
depends_on: [ARCH-007, ARCH-009]
---

## Description

Layered layout that places ingress on the left, compute in the middle and data on the right within containers, used after import and on demand.

## Acceptance criteria

- [ ] Auto layout produces no overlaps for the sample
- [ ] Manual positions are kept unless the user asks to re-layout
- [ ] Layout respects container nesting
