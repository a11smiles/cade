---
id: ARCH-012
title: Apply Azure naming conventions and abbreviations
tags: [arch, feature, rust]
epic: ARCH
priority: P1
depends_on: [ARCH-001]
---

## Description

Generate resource names from a pattern such as `{abbr}-{workload}-{env}-{region}` using the Cloud Adoption Framework abbreviations, with per-type length and character rules. Orgs can override the pattern in policy.

## Acceptance criteria

- [ ] New resources get a valid generated name
- [ ] Names are validated against each type's length and allowed characters
- [ ] Changing the workload or environment renames all resources in one undoable step
- [ ] Global-uniqueness types (storage accounts, Key Vault) warn that the name must be unique
