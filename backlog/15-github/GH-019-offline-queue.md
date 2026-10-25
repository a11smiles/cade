---
id: GH-019
title: Queue GitHub writes while offline
tags: [gh, feature, rust]
epic: GH
priority: P1
depends_on: [GH-018]
design: design/exports/v0001-2026-10-25/States.dc.html
---

## Description

Pushes, PR comments and feedback issues made offline are queued locally and sent when connectivity returns, with the user able to review or cancel them.

## Acceptance criteria

- [ ] Queued items are visible and cancelable
- [ ] Items send in order on reconnect
- [ ] Failures after reconnect surface with retry
