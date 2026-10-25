---
id: CORE-019
title: Commit project changes to the local git repo
tags: [core, feature, rust]
epic: CORE
priority: P0
depends_on: [CORE-015, CORE-018, GH-005]
design: design/exports/v0001-2026-10-25/History.dc.html
---

## Description

Projects are git repos (via `cade-git` on gix). CADE commits on explicit checkpoint, on applying a Copilot run, on ship, and on import, with structured messages (`copilot: <summary>`, `you: <summary>`, `import: <source>`) and the origin recorded in a trailer. History reads these commits.

## Acceptance criteria

- [ ] Applying a Copilot run creates one commit with the prompt summary and token count trailer
- [ ] Commits are attributed to the signed-in GitHub identity
- [ ] Uncommitted changes are always recoverable from the journal
- [ ] Projects opened from an existing repo don't rewrite history

## Notes

Branch/PR/push flows are in GH. History UI is in HIST.
