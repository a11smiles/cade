---
id: DS-022
title: Keep the design-system artifact and tokens.json in sync
tags: [ds, chore]
epic: DS
priority: P2
depends_on: [DS-001]
---

## Description

The CADE design system is also published as a browsable reference. Add a script that exports `tokens.json` to the reference format and a check that flags drift between the two.

## Acceptance criteria

- [ ] Export script writes the reference tokens from `tokens.json`
- [ ] Drift check lists tokens that differ
- [ ] Process is documented in `design/system/README.md`
