---
id: DECK-018
title: Add the pre-export fidelity check for .pptx
tags: [deck, feature, ui]
epic: DECK
priority: P0
depends_on: [DECK-017]
design: design/exports/v0001-2026-10-25/Slides.dc.html
---

## Description

Before export, run a check and show it in the Copilot column card 'Export check · .pptx': text stays
editable, fonts embedded or mapped, and a warning for each element that will export as an image (e.g. linked
phone mock on slide 3). Users can fix, accept, or ask Copilot to rebuild the element natively.

## Acceptance criteria

- [ ] Check runs in under 2 seconds for a 50-slide deck
- [ ] Each warning links to the slide and element it refers to
- [ ] Fonts that cannot be embedded are listed with their fallback mapping
- [ ] Export can proceed with warnings; errors (corrupt media) block until fixed
