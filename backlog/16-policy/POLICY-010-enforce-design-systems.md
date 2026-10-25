---
id: POLICY-010
title: Enforce approved design systems in outputs
tags: [policy, feature, rust]
epic: POLICY
priority: P1
depends_on: [POLICY-002, CORE-008]
design: design/exports/v0001-2026-10-25/Ship.dc.html
---

## Description

`outputs.design_systems` lists allowed systems (e.g. `contoso-ds@4`). Only those can be installed in projects; when 'design system only' is on, raw colors/sizes in App outputs fail the Ship pre-flight check ('41 layers map to @contoso/ds; no raw hex values').

## Acceptance criteria

- [ ] Installing a non-approved design system is blocked with the rule
- [ ] Ship pre-flight lists layers with raw values and blocks shipping
- [ ] Copilot write tools refuse raw values (COPILOT-019)
- [ ] Major version pin (`@4`) rejects v5 until policy updates
