---
id: DOC-004
title: Bind document text styles to design-system tokens
tags: [doc, feature, rust]
epic: DOC
priority: P0
depends_on: [DOC-001, CORE-007]
design: design/exports/v0001-2026-10-25/Document.dc.html
---

## Description

Text styles (Title, Heading, Body, Callout, Caption, Quote) come from the active design system with
size, line height, weight and color as token references. The 'Text styles · Contoso DS' panel lists them
with their sizes. Changing a token in the design system updates every document that uses it.

## Acceptance criteria

- [ ] Applying a style stores a style reference, not copied values
- [ ] Updating a type token re-styles all affected text and re-paginates
- [ ] Local overrides are visible in the strip and can be reset to the style
- [ ] Documents without a design system fall back to CADE's built-in styles
