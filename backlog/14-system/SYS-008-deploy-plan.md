---
id: SYS-008
title: Compute a deploy plan per environment
tags: [sys, feature, rust]
epic: SYS
priority: P0
depends_on: [SYS-007]
design: design/exports/v0001-2026-10-25/Deploy.dc.html
---

## Description

Compute ordered steps for the selected environment: infra what-if/apply, migrations, app build and deploy to slot, smoke tests and swap, skipping steps with no changes.

## Acceptance criteria

- [ ] Plan lists only steps with changes
- [ ] Steps are ordered so migrations run before the new app version starts
- [ ] Plan explains blockers (e.g. missing setting) before PR
