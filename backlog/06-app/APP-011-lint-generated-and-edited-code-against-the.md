---
id: APP-011
title: Lint generated and edited code against the design system
tags: [app, rust, feature, policy]
epic: APP
priority: P0
depends_on: [APP-005]
design: design/exports/v0001-2026-10-25/Ship.dc.html
---

## Description

DS lint flags raw values where tokens exist, components not from the approved library, and inaccessible patterns. It runs in Code mode, before Ship, and as an agent self-check tool.

## Acceptance criteria

- [ ] Raw hex colors, px spacing and font sizes with token equivalents are flagged
- [ ] Imports of non-approved UI libraries are flagged
- [ ] DS lint result shows in the Code status bar and Ship pre-flight
- [ ] Copilot can call DS lint as a tool and fix its own violations
