---
id: ONB-018
title: Persist settings with policy overrides applied
tags: [onb, chore, rust]
epic: ONB
priority: P0
depends_on: [ONB-008, POLICY-004]
---

## Description

Store user settings in a versioned JSON file in the OS config dir with schema migrations. The effective setting is `policy lock > user value > default`; locked values are never written over the user's choice so they return if the policy is lifted.

## Acceptance criteria

- [ ] Settings survive restart and app updates
- [ ] Schema migration test from v1 to v2 passes
- [ ] A locked setting shows policy value but preserves the stored user value
- [ ] Corrupt settings file is backed up and reset with a notice
