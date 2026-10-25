---
id: ONB-010
title: Implement Settings › Copilot & models
tags: [onb, feature, ui]
epic: ONB
priority: P0
depends_on: [ONB-008, COPILOT-011, COPILOT-031]
design: design/exports/v0001-2026-10-25/Settings.dc.html
---

## Description

Provider cards (GitHub Copilot / Bring your own key with lock when disabled), default model by output type, BYOK provider management (add/edit/remove/test), and Usage this month in tokens with breakdown by output type and reset date.

## Acceptance criteria

- [ ] Provider selection switches the active provider for new runs
- [ ] BYOK card shows the policy lock when disabled
- [ ] Usage section shows tokens used, budget and reset date
- [ ] Removing a BYOK provider deletes its secrets
