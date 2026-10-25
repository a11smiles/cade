---
id: DOC-009
title: Embed live linked frames in documents
tags: [doc, feature, rust]
epic: DOC
priority: P0
depends_on: [DOC-001, DECK-016]
design: design/exports/v0001-2026-10-25/Document.dc.html
---

## Description

Reuse the live linked frame mechanism to place App frames (e.g. 'Linked · Step 1 · Mobile') in a document
with a caption. Supports pin-to-version and break link. On export, linked frames render as vector in PDF
and high-resolution images in .docx.

## Acceptance criteria

- [ ] Linked frames update when the source changes
- [ ] Pinned versions stay fixed across source edits
- [ ] PDF export keeps linked frames as vector
- [ ] Broken links are flagged in the print check
