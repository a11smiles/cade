---
id: IMPORT-015
title: Convert Figma components, variants and instances
tags: [import, feature, rust]
epic: IMPORT
priority: P0
depends_on: [IMPORT-014, CORE-009]
design: design/exports/v0001-2026-10-25/Import.dc.html
---

## Description

Components and component sets become CADE components with variant properties; instances keep their
overrides; component properties (boolean, text, instance swap) map to CADE component properties.

## Acceptance criteria

- [ ] Instances update when their imported main component changes
- [ ] Overrides survive import
- [ ] Variant properties appear in the inspector with the same names
- [ ] Detached instances import as plain layers and are reported
