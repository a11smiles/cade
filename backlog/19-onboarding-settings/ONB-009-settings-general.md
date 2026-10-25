---
id: ONB-009
title: Implement Settings › General
tags: [onb, feature, ui]
epic: ONB
priority: P1
depends_on: [ONB-008]
design: design/exports/v0001-2026-10-25/Settings.dc.html
---

## Description

Projects folder, open-at-launch behavior (Start screen or last project), language (English first; strings externalized), update channel (stable/beta, from FOUND auto-update), and 'Commit Copilot threads to project repos' default.

## Acceptance criteria

- [ ] Changing the projects folder updates Start's recent list
- [ ] Open-at-launch setting is honored
- [ ] All UI strings come from the i18n catalog
- [ ] Update channel switch takes effect on next update check
