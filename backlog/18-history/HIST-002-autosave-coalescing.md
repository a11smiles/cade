---
id: HIST-002
title: Autosave and coalesce manual edits into versions
tags: [hist, history, feature, rust]
epic: HIST
priority: P0
depends_on: [HIST-001, CORE-017]
---

## Description

Loro saves continuously to disk; versions are cut as commits after 2 minutes of idle or 50 ops, whichever first, labelled with a generated summary ('Button radius 6 → 8 on Continue'). Users can also save a named version (Cmd/Ctrl+S with a name).

## Acceptance criteria

- [ ] Idle 2 minutes after edits produces one commit
- [ ] Rapid edits coalesce into one version, not dozens
- [ ] Named versions use the given name as the commit subject
- [ ] Crash during edit loses at most the last second of work (Loro journal)
