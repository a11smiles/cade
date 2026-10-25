---
id: DOC-001
title: Define the document model: flow, pages and page settings
tags: [doc, feature, rust]
epic: DOC
priority: P0
depends_on: [CORE-003]
design: design/exports/v0001-2026-10-25/Document.dc.html
---

## Description

Add a `Document` output type to `cade-core`. A document is a flow of blocks (paragraphs, headings,
callouts, tables, images, stat rows, linked frames) plus section settings: page size (Letter, A4, Legal,
custom), orientation, margins, columns and header/footer. Pages are computed by the pagination engine,
not stored, so concurrent edits never conflict over page breaks.

## Acceptance criteria

- [ ] A document with 40 blocks round-trips through save and reopen
- [ ] Page size and margins are per section and default from the template
- [ ] Pages are derived on load; no page-break state is persisted except explicit manual breaks
- [ ] Two peers editing different paragraphs converge without lost text
