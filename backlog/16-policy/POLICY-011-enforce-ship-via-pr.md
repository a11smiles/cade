---
id: POLICY-011
title: Require ship and deploy to go through pull requests
tags: [policy, feature, rust]
epic: POLICY
priority: P0
depends_on: [POLICY-002, GH-010]
design: design/exports/v0001-2026-10-25/Ship.dc.html
---

## Description

With `outputs.ship_via: pull_request`, CADE never pushes to protected or default branches directly: Ship creates a branch and PR, Deploy writes a workflow and PR. Direct push options are hidden.

## Acceptance criteria

- [ ] No code path pushes to the default branch when the rule is on (integration test with a mock remote)
- [ ] Ship always creates a `cade/*` branch and PR
- [ ] Deploy only generates workflow files and opens a PR
- [ ] Rule appears in the Ship screen's policy badge
