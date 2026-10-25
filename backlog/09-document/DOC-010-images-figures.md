---
id: DOC-010
title: Add images and figures with captions
tags: [doc, feature, rust]
epic: DOC
priority: P1
depends_on: [DOC-002]
design: design/exports/v0001-2026-10-25/Document.dc.html
---

## Description

Inline and floating images with crop, captions, figure numbering and alt text. Text wraps around floating
images. Resolution is tracked for the print check.

## Acceptance criteria

- [ ] Captions number automatically (Figure 1, 2…) and update on reorder
- [ ] Floating images support square and top-bottom wrap
- [ ] Alt text is stored and exported to tagged PDF and .docx
- [ ] Effective print resolution is computed per image
