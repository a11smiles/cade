---
id: APP-028
title: Persist App workspace state per project
tags: [app, rust, ts, chore]
epic: APP
priority: P1
depends_on: [APP-002]
---

## Description

Remember mode, open pages, viewport, panel sizes, editor tabs and Copilot thread per project across restarts.

## Acceptance criteria

- [ ] Reopening a project restores mode, viewport, panels and tabs
- [ ] State is stored outside the repo (not committed)
- [ ] Corrupt state files are ignored without blocking the project from opening
