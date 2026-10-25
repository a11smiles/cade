---
id: REL-012
title: Version design exports in the repo
tags: [rel, chore]
epic: REL
priority: P0
depends_on: []
---

## Description

Design exports live in `design/exports/vNNNN-YYYY-MM-DD/`, one folder per export, never overwritten. `design/exports/LATEST` names the current folder and `design/exports/CHANGELOG.md` records what changed per export. Backlog items reference a specific export version so implementation targets don't shift silently.

## Acceptance criteria

- [ ] Export folders follow the naming rule and are immutable once merged (CI check)
- [ ] LATEST and CHANGELOG are updated with each new export
- [ ] A script lists backlog items whose `design:` path points at an older export than LATEST
