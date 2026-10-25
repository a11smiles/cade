---
id: IMPORT-019
title: Map Figma fonts to available fonts
tags: [import, feature, rust]
epic: IMPORT
priority: P1
depends_on: [IMPORT-014]
design: design/exports/v0001-2026-10-25/Import.dc.html
---

## Description

Detect fonts used in the file, match them to installed fonts, bundled Google Fonts and design-system fonts,
and offer substitutions for missing ones before import completes.

## Acceptance criteria

- [ ] Missing fonts are listed with suggested substitutes
- [ ] Substitutions are stored so re-imports reuse them
- [ ] Text metrics differences are flagged on affected frames
