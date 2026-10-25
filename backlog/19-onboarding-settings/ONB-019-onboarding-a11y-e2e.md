---
id: ONB-019
title: Test onboarding and settings end to end, with accessibility
tags: [onb, test, a11y]
epic: ONB
priority: P1
depends_on: [ONB-002, ONB-013]
design: design/exports/v0001-2026-10-25/Onboarding.dc.html
---

## Description

Automated end-to-end tests (Tauri WebDriver) for first run, sign-in with a mocked device flow, policy load, folder selection and each settings section, plus axe-core accessibility checks on every screen in dark and light themes.

## Acceptance criteria

- [ ] E2E suite covers all onboarding steps and settings sections
- [ ] axe-core reports zero serious violations in both themes
- [ ] Suite runs on all three OSes in CI
- [ ] Mock GitHub server is reusable by other epics
