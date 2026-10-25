---
id: DOC-016
title: Export documents to tagged PDF
tags: [doc, feature, rust, a11y]
epic: DOC
priority: P0
depends_on: [DOC-002, DOC-010]
design: design/exports/v0001-2026-10-25/Document.dc.html
---

## Description

PDF export via `cade-io-export`: embedded font subsets, vector graphics, selectable text, document
outline from headings, tagged PDF structure for accessibility (headings, lists, tables, alt text), optional
PDF/A-2b and optional bleed.

## Acceptance criteria

- [ ] Exported PDFs pass PAC (PDF Accessibility Checker) structure checks for the reference document
- [ ] Headings appear as PDF bookmarks
- [ ] PDF/A-2b output validates with veraPDF
- [ ] Exports match canvas rendering in snapshot tests
