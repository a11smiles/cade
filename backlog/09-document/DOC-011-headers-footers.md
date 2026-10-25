---
id: DOC-011
title: Add headers, footers, page numbers and classification labels
tags: [doc, feature, rust]
epic: DOC
priority: P1
depends_on: [DOC-002]
design: design/exports/v0001-2026-10-25/Document.dc.html
---

## Description

Per-section header and footer areas with fields: page number, page count, document title, date and a
classification label (e.g. 'Internal') that can be required by org policy. First-page-different option.

## Acceptance criteria

- [ ] Page number and count fields update after re-pagination
- [ ] First-page-different hides or replaces the header on page 1
- [ ] Policy can require a classification label; the print check fails without it
- [ ] Fields export to .docx as live Word fields
