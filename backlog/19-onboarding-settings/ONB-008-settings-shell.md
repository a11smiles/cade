---
id: ONB-008
title: Build the Settings shell with navigation and search
tags: [onb, feature, ui, ts]
epic: ONB
priority: P0
depends_on: [DS-010]
design: design/exports/v0001-2026-10-25/Settings.dc.html
---

## Description

Settings window/route with left navigation (General, Copilot & models, Agent permissions, Design systems, Appearance, Keyboard shortcuts, Azure subscriptions, Privacy, About, Org policy) and a search box that filters settings by label and keywords and jumps to them.

## Acceptance criteria

- [ ] Every section is reachable from the nav and deep links (`cade://settings/appearance`)
- [ ] Search finds settings by label and synonyms
- [ ] Admin entry appears only for admins (POLICY-018)
- [ ] Settings open from the gear icon in every workspace and from the app menu
