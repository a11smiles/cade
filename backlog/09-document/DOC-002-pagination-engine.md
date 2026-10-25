---
id: DOC-002
title: Build the incremental pagination engine
tags: [doc, feature, rust, perf]
epic: DOC
priority: P0
depends_on: [DOC-001, CANVAS-004]
design: design/exports/v0001-2026-10-25/Document.dc.html
---

## Description

Lay out the block flow into pages using Parley for text. Supports widow/orphan control, keep-with-next,
keep-together, manual page breaks, repeating table headers and block splitting rules (paragraphs split by
line, tables by row, callouts never split unless taller than a page). Re-paginates incrementally from the
first changed block.

## Acceptance criteria

- [ ] Typing in a 30-page document re-paginates in under 16 ms per keystroke
- [ ] Widows and orphans are prevented with the default 2-line rule
- [ ] Keep-with-next keeps a heading or callout on the same page as the following block
- [ ] Table header rows repeat on continuation pages
- [ ] Golden-file tests cover splitting for each block type
