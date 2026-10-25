---
id: DOC-019
title: Expose document editing as Copilot tools
tags: [doc, feature, rust]
epic: DOC
priority: P0
depends_on: [COPILOT-020, DOC-006]
design: design/exports/v0001-2026-10-25/Document.dc.html
---

## Description

Agent tools: rewrite selected block for an audience, adjust tone, shorten to fit the page, generate an
outline, fill placeholders from attached sources, and fix print check findings. Rewrites show a preview with
'Replace' and 'Try another'.

## Acceptance criteria

- [ ] Rewrites of a selected callout change only that block
- [ ] 'Replace' applies the preview in one undo step; 'Try another' generates a new variant
- [ ] Copilot respects page fit ('kept it to two lines so page 1 still fits') by checking pagination
- [ ] Placeholders are only filled from provided sources, never invented
