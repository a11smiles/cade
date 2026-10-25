---
id: COPILOT-021
title: Stage agent edits as change sets on a Loro fork
tags: [copilot, feature, rust]
epic: COPILOT
priority: P0
depends_on: [CORE-013, COPILOT-002]
design: design/exports/v0001-2026-10-25/Copilot.dc.html
---

## Description

A run forks the project's Loro document; all agent writes land on the fork and are tracked as a change set of ops grouped by layer (ADD/MOD/DEL with before/after props). Applying merges the fork's ops into the main doc as one transaction; discarding drops the fork. If the user edits the main doc during a run, the merge is still conflict-free because of the CRDT, and changed-under-you layers are flagged.

## Acceptance criteria

- [ ] A run's change set lists every touched layer with ADD/MOD/DEL and property-level before/after
- [ ] Apply merges in one undo step; Undo after apply restores the exact pre-run state
- [ ] User edits made during a run are preserved after apply (CRDT merge test)
- [ ] Layers edited by both user and agent are flagged 'Changed while Copilot worked' in review
- [ ] Discard leaves the main document byte-identical to before the run
