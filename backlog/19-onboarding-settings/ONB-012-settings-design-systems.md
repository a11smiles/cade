---
id: ONB-012
title: Implement Settings › Design systems
tags: [onb, feature, ui]
epic: ONB
priority: P1
depends_on: [ONB-008, DS-001, POLICY-010]
design: design/exports/v0001-2026-10-25/Settings.dc.html
---

## Description

List installed design systems (name, version, source repo, approved badge), set the default for new projects, install from a repo or local folder, update to newer versions, and remove. Non-approved systems are blocked when policy requires approval.

## Acceptance criteria

- [ ] Installing from a repo URL pulls tokens and components
- [ ] Default system is applied to new projects
- [ ] Update shows a changelog when available
- [ ] Non-approved installs are blocked with the rule
