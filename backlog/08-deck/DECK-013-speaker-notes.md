---
id: DECK-013
title: Add per-slide speaker notes with Copilot drafting
tags: [deck, feature, ui]
epic: DECK
priority: P0
depends_on: [DECK-001]
design: design/exports/v0001-2026-10-25/Slides.dc.html
---

## Description

Notes panel under the stage with rich text (bold, lists, links). Copilot can draft notes for one slide or
the whole deck, marked 'Drafted by Copilot · edit freely' until the user edits them. Notes are included in
presenter view, PDF handouts and .pptx export.

## Acceptance criteria

- [ ] Notes persist per slide and appear in presenter view
- [ ] 'Write speaker notes for all slides' fills empty notes only, never overwriting user-written notes
- [ ] Copilot-drafted notes show a label that clears on first user edit
- [ ] Notes export to .pptx notes pages
