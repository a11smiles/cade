---
id: DOC-018
title: Export documents to Markdown
tags: [doc, feature, rust]
epic: DOC
priority: P1
depends_on: [DOC-001]
design: design/exports/v0001-2026-10-25/Document.dc.html
---

## Description

GitHub Flavored Markdown export: headings, lists, tables, links, images written to an `assets/` folder,
callouts as blockquotes with a GitHub alert type where it fits (NOTE, WARNING, CAUTION), stat rows as tables.

## Acceptance criteria

- [ ] Output renders correctly on github.com
- [ ] Images are written next to the .md file with relative links
- [ ] Callout tones map to GitHub alert types
