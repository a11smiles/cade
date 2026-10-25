---
id: DECK-019
title: Embed or map fonts for .pptx and PDF export
tags: [deck, feature, rust]
epic: DECK
priority: P1
depends_on: [DECK-017]
design: design/exports/v0001-2026-10-25/Slides.dc.html
---

## Description

Embed font subsets where the font license allows embedding (fsType), otherwise map to a configured
fallback (per design system) and record it in the export check. Google Fonts such as Geist are bundled
with their licenses.

## Acceptance criteria

- [ ] Fonts with restricted embedding permissions are never embedded
- [ ] Design systems can declare fallback fonts per family
- [ ] The export check lists every mapped font
- [ ] Exported PDFs contain only the glyph subsets used
