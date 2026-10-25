---
id: ONB-001
title: Build the first-run wizard shell
tags: [onb, feature, ui, ts]
epic: ONB
priority: P0
depends_on: [DS-010, FOUND-003]
design: design/exports/v0001-2026-10-25/Onboarding.dc.html
---

## Description

Two-column first-run screen: brand side (CADE · Copilot ADE, value statement, output-type chips) and a 3-step setup side (Sign in → Connecting to Copilot → Projects folder) with progress ('Step 2 of 3'). Shown on first launch and whenever no account is signed in.

## Acceptance criteria

- [ ] First launch shows the wizard; later launches go to Start
- [ ] Steps advance only when their checks pass
- [ ] Wizard is fully keyboard-operable and screen-reader labelled
- [ ] Window can be resized to 1024×700 without clipping
