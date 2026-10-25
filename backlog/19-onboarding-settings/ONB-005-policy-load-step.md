---
id: ONB-005
title: Load org policy during onboarding with a summary
tags: [onb, feature, ui]
epic: ONB
priority: P0
depends_on: [POLICY-003, POLICY-017]
design: design/exports/v0001-2026-10-25/Onboarding.dc.html
---

## Description

Show 'Loading <Org> policy' with a spinner, then the 'Set by your organization' amber card summarizing user-visible rules (telemetry, shell, network) and a link to all rules in Settings.

## Acceptance criteria

- [ ] Card appears only when a policy applies
- [ ] Summary text is generated from the effective policy, not hard-coded
- [ ] Policy load failure allows continuing with cached or default policy and says which
- [ ] Link opens Settings › Org policy
