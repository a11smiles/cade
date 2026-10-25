---
id: GH-021
title: Initialize new project repos with CADE files
tags: [gh, feature, rust]
epic: GH
priority: P1
depends_on: [GH-008]
---

## Description

New repos created from CADE get a README, `.gitattributes` for `.cade` files, LFS config for assets, CODEOWNERS stub and the design folder layout.

## Acceptance criteria

- [ ] New repos include the standard files
- [ ] .cade files are marked binary-merge with a custom driver
- [ ] LFS is configured for image and video assets
