---
id: POLICY-019
title: Edit policy rules in the admin console
tags: [policy, feature, ui]
epic: POLICY
priority: P1
depends_on: [POLICY-018, POLICY-001]
design: design/exports/v0001-2026-10-25/Admin.dc.html
---

## Description

Form-based editor for every v1 rule (models, agent tools, output rules, visibility and privacy, budgets) that edits a working copy of `policy.yml`. The right panel shows the YAML with live diff against the current version.

## Acceptance criteria

- [ ] Every schema key has a control
- [ ] Edits update the YAML preview and diff immediately
- [ ] Invalid combinations are blocked with inline errors
- [ ] Discard resets to the current committed policy
