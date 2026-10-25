---
id: ARCH-005
title: Map Azure resource types to icon files
tags: [arch, feature, rust]
epic: ARCH
priority: P0
depends_on: [ARCH-004, ARCH-002]
---

## Description

Ship `assets/icons/azure-map.yaml` that maps resource types (and some kinds, e.g. Function App vs Web App on `Microsoft.Web/sites`) to icon file names in the official pack. Unmapped types use a fallback tile showing the service abbreviation, so a missing mapping never blocks design.

## Acceptance criteria

- [ ] Mapping covers at least the 150 most used resource types at 1.0
- [ ] Kind-aware mapping picks the Function App icon when `kind` contains `functionapp`
- [ ] A CI check lists catalog types with no mapping and fails if a mapped file name is missing from the pack listing
- [ ] Fallback tile uses the type's short name and the Azure category color token

## Notes

Pack file names change between releases; key the map on a normalized name and keep an alias list.
