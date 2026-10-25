---
id: GH-009
title: Run pre-flight checks before anything leaves the machine
tags: [gh, feature, rust, security]
epic: GH
priority: P0
depends_on: [GH-008]
design: design/exports/v0001-2026-10-25/Ship.dc.html
---

## Description

Before push, run checks locally: design-system-only usage, approved packages, secret scanning of generated files and notes, accessibility checks, IaC lint and PII classification when relevant. Policy decides which checks block.

## Acceptance criteria

- [ ] All checks run locally with no network calls except package allowlist lookups from policy cache
- [ ] Blocking failures disable Ship with reasons
- [ ] Secret scanning uses a bundled ruleset and finds common token formats
- [ ] Results are included in the PR description
