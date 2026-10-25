---
id: DS-020
title: Publish a component catalog with a11y checks
tags: [ds, chore, ui, a11y]
epic: DS
priority: P1
depends_on: [DS-007]
---

## Description

Run Ladle or Storybook for `app/ui` components with a theme toggle and axe accessibility checks on every story. Deploy the catalog to the docs site.

## Acceptance criteria

- [ ] Every DS component has at least one story per variant
- [ ] axe runs on every story in CI with zero serious violations
- [ ] Catalog is published with the docs site on each release
