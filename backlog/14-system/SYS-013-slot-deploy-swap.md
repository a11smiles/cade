---
id: SYS-013
title: Deploy to a staging slot and swap
tags: [sys, feature, rust]
epic: SYS
priority: P1
depends_on: [SYS-009, ARCH-013]
design: design/exports/v0001-2026-10-25/Deploy.dc.html
---

## Description

For App Service targets, deploy to a staging slot, run smoke tests and swap; for other hosting (Container Apps) use revisions with traffic shifting.

## Acceptance criteria

- [ ] App Service deploys use a slot and swap
- [ ] Container Apps use revision traffic splitting
- [ ] Failed smoke tests leave production untouched
