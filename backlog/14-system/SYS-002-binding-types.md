---
id: SYS-002
title: Define binding types between layers
tags: [sys, feature, rust]
epic: SYS
priority: P0
depends_on: [SYS-001]
design: design/exports/v0001-2026-10-25/Deploy.dc.html
---

## Description

Bindings: data field → app component; migration → database resource; app build → hosting resource; app setting → resource output or Key Vault secret; app runtime → hosting stack. Each binding knows how to check itself.

## Acceptance criteria

- [ ] Each binding type has a check returning in sync, pending or broken with a reason
- [ ] Bindings are created automatically where unambiguous and suggested otherwise
- [ ] Users can create and remove bindings manually
