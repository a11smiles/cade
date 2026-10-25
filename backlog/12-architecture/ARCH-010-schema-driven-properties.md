---
id: ARCH-010
title: Generate the properties strip from the resource schema
tags: [arch, feature, ui, ts]
epic: ARCH
priority: P0
depends_on: [ARCH-002, DS-011]
design: design/exports/v0001-2026-10-25/Architecture.dc.html
---

## Description

When a resource is selected, the floating properties strip shows its most important settings (SKU, region, scale, public access, identity) with an expandable full form generated from the catalog schema. Enums become dropdowns, booleans toggles, and read-only fields are hidden.

## Acceptance criteria

- [ ] Every writable property of the selected API version is editable in the full form
- [ ] Curated "top properties" per common resource type appear in the strip
- [ ] Invalid values are flagged inline with the schema's constraint
- [ ] Changing a property updates the code drawer within 200 ms
- [ ] Secure-by-default suggestions (HTTPS only, TLS 1.2+, public access off) are shown and applied on request
