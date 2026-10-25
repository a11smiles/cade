---
id: POLICY-017
title: Show the effective policy and lock badges in Settings
tags: [policy, feature, ui]
epic: POLICY
priority: P0
depends_on: [POLICY-004]
design: design/exports/v0001-2026-10-25/Settings.dc.html
---

## Description

Add Settings › Org policy: effective rules grouped like the policy file, each with source org and commit, plus lock badges on any setting a rule controls ('Turned off by Contoso policy'). Onboarding shows a short summary ('Set by your organization: telemetry off, shell asks first, network off').

## Acceptance criteria

- [ ] Every locked setting shows a lock and links to its rule
- [ ] Org policy page shows source repo, commit and fetch time
- [ ] Onboarding summary lists the three most user-visible rules
- [ ] Users without org policy see 'Using CADE defaults'
