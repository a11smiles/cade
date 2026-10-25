---
id: APP-004
title: Define the component-to-code mapping for a design system
tags: [app, rust, feature]
epic: APP
priority: P0
depends_on: [APP-003]
design: design/exports/v0001-2026-10-25/Ship.dc.html
---

## Description

Each library component maps to a real code import (e.g. Button → `@contoso/ds` Button) with prop mappings for variants. The mapping ships with the library and lets codegen emit real components, not look-alikes.

## Acceptance criteria

- [ ] A mapping file format is defined and documented
- [ ] Variant properties map to component props
- [ ] Unmapped components are flagged before codegen
- [ ] Ship pre-flight shows the count of layers mapped to real components
