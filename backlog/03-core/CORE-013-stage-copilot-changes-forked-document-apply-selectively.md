---
id: CORE-013
title: Stage Copilot changes in a forked document and apply selectively
tags: [core, feature, rust]
epic: CORE
priority: P0
depends_on: [CORE-002, CORE-011, CORE-014]
design: design/exports/v0001-2026-10-25/Copilot.dc.html
---

## Description

Agent runs write to a fork of the document. The fork's diff against the main document is shown as a staged change set. The user can apply all, apply some, or discard. Applying merges only the chosen changes into the main document under the run's origin. Main-document edits made during the run are kept.

## Acceptance criteria

- [ ] During a run the main document is unchanged until apply
- [ ] Applying a subset merges only those changes
- [ ] Edits the user makes during the run survive the merge
- [ ] Discarding leaves no trace in the main document
- [ ] Conflicting edits to the same property are surfaced for the user to choose
