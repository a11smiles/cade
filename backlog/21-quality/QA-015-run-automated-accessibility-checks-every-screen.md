---
id: QA-015
title: Run automated accessibility checks on every screen
tags: [qa, test, a11y]
epic: QA
priority: P1
depends_on: [QA-007]
---

## Description

Run axe-core inside e2e tests for each workspace in dark and light themes and fail on serious or critical violations.

## Acceptance criteria

- [ ] Every workspace is scanned in both themes
- [ ] Serious and critical violations fail CI
- [ ] Known exceptions are listed with justification
