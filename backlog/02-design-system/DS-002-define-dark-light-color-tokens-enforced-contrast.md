---
id: DS-002
title: Define dark and light color tokens with enforced contrast
tags: [ds, feature, ui, a11y]
epic: DS
priority: P0
depends_on: [DS-001]
design: design/exports/v0001-2026-10-25/Start.dc.html
---

## Description

Port the palette used in the v0001 screens into semantic tokens: graphite surfaces, violet accent for Copilot and selection, amber for governance, green for success, red for danger, blue for Architecture, teal for Data model. Every text/background pair used by components is listed and checked automatically for WCAG 2.2 AA in both themes.

## Acceptance criteria

- [ ] Every text token pairs with its intended surface at 4.5:1 or better (3:1 for large text and UI glyphs)
- [ ] Contrast check runs in CI over the declared pairs for dark and light
- [ ] Status colors also differ in lightness so they're distinguishable without hue
- [ ] Type tags (APP, IMAGE, DECK, DOCUMENT, MOTION, DIAGRAM, ARCHITECTURE, DATA MODEL, SYSTEM, ADMIN) each have a token pair
