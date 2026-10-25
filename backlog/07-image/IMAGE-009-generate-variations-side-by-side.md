---
id: IMAGE-009
title: Generate variations side by side
tags: [image, rust, feature]
epic: IMAGE
priority: P1
depends_on: [COPILOT-020, IMAGE-003]
design: design/exports/v0001-2026-10-25/Image.dc.html
---

## Description

'Show 3 variations' asks Copilot for alternative versions of the selection or image, laid out next to the original as temporary artboards. Pick one to apply, or keep several as alternates.

## Acceptance criteria

- [ ] Variations appear as temporary artboards labeled A, B, C
- [ ] Applying a variation replaces the original in one undo step
- [ ] Unpicked variations can be discarded or kept
