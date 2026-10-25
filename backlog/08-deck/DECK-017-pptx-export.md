---
id: DECK-017
title: Export decks to .pptx with editable text, shapes and charts
tags: [deck, feature, rust]
epic: DECK
priority: P0
depends_on: [DECK-003, DECK-008, DECK-013]
design: design/exports/v0001-2026-10-25/Slides.dc.html
---

## Description

`cade-io-pptx` writer: masters and layouts to slideMasters/slideLayouts, text to editable text frames with
runs and paragraph styles, shapes to preset or custom geometry, images to media parts, charts to native
PowerPoint charts with embedded workbook data, tables to native tables, and notes to notesSlides. Content
with no PowerPoint equivalent (linked frames, effects) exports as high-resolution images.

## Acceptance criteria

- [ ] Exported files open without repair prompts in PowerPoint (Windows, macOS), Keynote and LibreOffice
- [ ] Titles and body text are editable text in PowerPoint, not images
- [ ] Charts open as editable PowerPoint charts with the original data
- [ ] Speaker notes appear in PowerPoint notes view
- [ ] A 50-slide deck exports in under 10 seconds
