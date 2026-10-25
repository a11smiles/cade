---
id: DOC-006
title: Add the callout block with tone tokens
tags: [doc, feature, rust]
epic: DOC
priority: P0
depends_on: [DOC-004]
design: design/exports/v0001-2026-10-25/Document.dc.html
---

## Description

Callout block with title and body, tone (info, warning, danger, neutral) bound to surface tokens,
width (full column or inset) and keep-with-next. Callouts never split across pages unless taller than a page.

## Acceptance criteria

- [ ] Tone swatches in the strip list only design-system surface tokens
- [ ] Callouts move whole to the next page instead of splitting
- [ ] Keep-with-next is on by default and can be turned off
- [ ] Callouts export to .docx as shaded paragraphs with the same colors
