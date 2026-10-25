---
id: SYS-006
title: Trace a change's impact across layers
tags: [sys, feature, rust]
epic: SYS
priority: P1
depends_on: [SYS-001]
design: design/exports/v0001-2026-10-25/Deploy.dc.html
---

## Description

Given a change (e.g. a new field), trace which app screens, migrations and resources are affected and present it in the System view and to Copilot.

## Acceptance criteria

- [ ] Selecting a pending change highlights affected items in all three cards
- [ ] Copilot can request an impact trace as a tool
- [ ] Trace output lists layer, item and expected action
