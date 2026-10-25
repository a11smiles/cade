---
id: DECK-024
title: Check slide reading order, alt text and contrast
tags: [deck, feature, a11y]
epic: DECK
priority: P1
depends_on: [DECK-017]
design: design/exports/v0001-2026-10-25/Slides.dc.html
---

## Description

Accessibility checker for decks: reading order per slide (editable), alt text for images, charts and
linked frames, title presence on every slide, and text contrast against slide backgrounds. Results feed the
export check, and reading order and alt text are written into .pptx and tagged PDF.

## Acceptance criteria

- [ ] Every slide without a title is flagged
- [ ] Images and charts without alt text are flagged; Copilot can draft alt text for review
- [ ] Reading order edits are reflected in exported .pptx
- [ ] Contrast below 4.5:1 for body text is flagged
