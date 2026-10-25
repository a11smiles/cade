---
id: DOC-005
title: Implement rich text editing with IME and spellcheck
tags: [doc, feature, rust]
epic: DOC
priority: P0
depends_on: [DOC-001, CANVAS-005]
design: design/exports/v0001-2026-10-25/Document.dc.html
---

## Description

Native text editing on the canvas: bold, italic, underline, links, inline code, bulleted and numbered lists
with nesting, IME composition for CJK input, bidirectional text, and OS spellcheck integration. Shared with
text layers in other outputs.

## Acceptance criteria

- [ ] Japanese, Chinese and Korean input via IME works on all three OSes
- [ ] Right-to-left paragraphs render and edit correctly
- [ ] Spelling errors are underlined using the OS spellchecker
- [ ] Copy/paste preserves formatting within CADE and to Word/Google Docs as rich text
- [ ] Undo groups typing into word-level steps
