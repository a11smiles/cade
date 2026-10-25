---
id: IMPORT-021
title: Import a Figma team library as a local design system
tags: [import, feature, rust]
epic: IMPORT
priority: P1
depends_on: [IMPORT-015, IMPORT-016]
design: design/exports/v0001-2026-10-25/Import.dc.html
---

## Description

Import a published library (components, styles, variables) into a CADE design system that can be published
to the org. Files imported later link to it instead of duplicating components.

## Acceptance criteria

- [ ] A library imports as a design system with components and tokens
- [ ] Later file imports link instances to the imported library
- [ ] The design system can be published for the org
