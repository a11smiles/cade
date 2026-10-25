---
id: CORE-008
title: Add project design-system libraries
tags: [core, feature, rust]
epic: CORE
priority: P0
depends_on: [CORE-007]
design: design/exports/v0001-2026-10-25/Main.dc.html
---

## Description

A project can attach one or more design-system libraries by reference (e.g. `contoso-ds@4`), each providing tokens, text styles, components and slide masters. Libraries are versioned; upgrading shows a diff of changed tokens and components before applying.

## Acceptance criteria

- [ ] A project can attach a library from a local folder or a git repo
- [ ] Library version is pinned in the project manifest
- [ ] Upgrade preview lists added, changed and removed tokens and components
- [ ] Removing a library leaves bound properties with their last resolved values and a warning
