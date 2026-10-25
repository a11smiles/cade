---
id: ARCH-011
title: Let users pick and upgrade resource API versions
tags: [arch, feature, rust]
epic: ARCH
priority: P1
depends_on: [ARCH-002, ARCH-010]
---

## Description

Each resource pins an API version, defaulting to the latest stable. Offer an upgrade action that shows which properties change, are added or are removed between versions.

## Acceptance criteria

- [ ] Default API version is the latest non-preview version in the catalog
- [ ] Upgrade dialog lists property differences before applying
- [ ] Preview API versions require an explicit opt-in and are flagged in the code drawer
- [ ] Policy can forbid preview API versions
