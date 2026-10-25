---
id: COPILOT-024
title: Undo or restore a whole Copilot run as one step
tags: [copilot, feature, rust]
epic: COPILOT
priority: P0
depends_on: [COPILOT-021, CORE-012]
design: design/exports/v0001-2026-10-25/Copilot.dc.html
---

## Description

After apply, the whole run is a single entry in undo history labelled with the run's summary ('Copilot: Added error states'). Undo reverts every op from the run, even if the user made unrelated edits afterwards (selective undo of the run's ops). Each applied run also creates a history commit (HIST-001).

## Acceptance criteria

- [ ] Undo immediately after apply restores the pre-run document
- [ ] Undoing a run after later manual edits reverts only the run's ops and keeps the manual edits
- [ ] Undo menu shows the run label
- [ ] Applied run produces exactly one history commit with the run id trailer
