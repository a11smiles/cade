---
id: CORE-026
title: Index large documents and load pages lazily
tags: [core, feature, rust, perf]
epic: CORE
priority: P1
depends_on: [CORE-003, CORE-005]
---

## Description

Maintain indexes by node kind, name, token usage and component usage for search, lint and the command palette. Pages load on demand so a 50-page project opens fast.

## Acceptance criteria

- [ ] Search by name across 10k nodes returns under 30 ms
- [ ] Opening a 50-page project renders the first page in under 1 s
- [ ] "Find all uses of token X" and "find instances of component Y" are supported
