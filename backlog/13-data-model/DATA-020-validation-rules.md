---
id: DATA-020
title: Define field validation rules shared with App
tags: [data, feature, rust]
epic: DATA
priority: P0
depends_on: [DATA-001]
design: design/exports/v0001-2026-10-25/Code.dc.html
---

## Description

Fields carry validation rules (required, min/max length, ranges, patterns, relative dates such as on-or-before today in the user's local date) that generate both database constraints where possible and a TypeScript validation module for App.

## Acceptance criteria

- [ ] Rules generate `validation.ts` used by App forms
- [ ] Rules that map to DB constraints generate CHECK constraints
- [ ] Changing a rule updates App validation and prototype behavior
- [ ] Relative date rules use the user's local date
