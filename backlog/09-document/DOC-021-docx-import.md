---
id: DOC-021
title: Import .docx into a document
tags: [doc, feature, rust]
epic: DOC
priority: P2
depends_on: [DOC-017]
design: design/exports/v0001-2026-10-25/Document.dc.html
---

## Description

Read .docx paragraphs, styles, lists, tables, images and headers/footers into the document model,
mapping Word styles to the nearest design-system styles and reporting anything approximated.

## Acceptance criteria

- [ ] Reference .docx imports with headings, lists and tables intact
- [ ] Unmapped Word styles are listed in the import report
- [ ] The original .docx file is never modified
