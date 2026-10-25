---
id: DECK-005
title: Support 16:9, 4:3 and custom slide sizes with rescale
tags: [deck, feature, rust]
epic: DECK
priority: P1
depends_on: [DECK-001]
design: design/exports/v0001-2026-10-25/Slides.dc.html
---

## Description

Header control for slide size. Changing size rescales content using layout constraints where present and
proportional scaling otherwise, with a choice of 'scale content' or 'keep size and re-flow'.

## Acceptance criteria

- [ ] Switching 16:9 to 4:3 offers scale or re-flow and the result is a single undo step
- [ ] Custom sizes accept px, in and cm and round-trip through .pptx export
- [ ] Constrained layers (pinned left/right) honor their constraints during re-flow
- [ ] Masters and layouts are resized together with slides
