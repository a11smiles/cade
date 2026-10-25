---
id: GH-012
title: Show GHAS code scanning status for the PR
tags: [gh, feature, rust, security]
epic: GH
priority: P1
depends_on: [GH-010]
design: design/exports/v0001-2026-10-25/Ship.dc.html
---

## Description

After the PR opens, poll checks and code scanning alerts for the PR and show status in CADE, including when CodeQL blocks merging.

## Acceptance criteria

- [ ] Check status and code scanning alert counts show in CADE
- [ ] Alerts link to GitHub
- [ ] Repos without GHAS show a neutral message
