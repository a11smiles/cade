---
id: DOC-015
title: Show print check results in the header and Copilot column
tags: [doc, feature, ui]
epic: DOC
priority: P0
depends_on: [DOC-014]
design: design/exports/v0001-2026-10-25/Document.dc.html
---

## Description

'Print check passed' pill in the header when clean, and a print check card listing passes and warnings
(e.g. '3 stat placeholders still to fill'). Clicking a finding selects the block. Copilot can propose fixes.

## Acceptance criteria

- [ ] Header pill reflects the current state and updates live
- [ ] Clicking a finding scrolls to and selects the block
- [ ] Copilot 'fix' proposals are staged for review
