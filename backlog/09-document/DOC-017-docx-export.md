---
id: DOC-017
title: Export documents to editable .docx
tags: [doc, feature, rust]
epic: DOC
priority: P0
depends_on: [DOC-004, DOC-008, DOC-011]
design: design/exports/v0001-2026-10-25/Document.dc.html
---

## Description

.docx writer mapping CADE text styles to named Word styles, lists to numbering definitions, tables to
Word tables, images with alt text, headers/footers with fields, and callouts to shaded paragraphs. Linked
frames export as images.

## Acceptance criteria

- [ ] Exported files open without repair in Word (Windows, macOS) and LibreOffice
- [ ] Word shows CADE styles in its Styles pane by name
- [ ] Lists and tables remain editable
- [ ] Round-trip check: re-importing into Word keeps page count within ±1 page
