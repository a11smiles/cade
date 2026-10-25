---
id: HIST-001
title: Commit every save, run, ship and import with structured trailers
tags: [hist, history, feature, rust]
epic: HIST
priority: P0
depends_on: [CORE-019, COPILOT-024]
design: design/exports/v0001-2026-10-25/History.dc.html
---

## Description

Every version-worthy event becomes a git commit in the project repo via `cade-git`: manual saves, applied Copilot runs, ships, imports and restores. Commits carry trailers (`CADE-Kind: copilot|you|ship|import|restore`, `CADE-Run-Id`, `CADE-Model`, `CADE-Tokens`, `CADE-Changes`) so history can be rebuilt from git alone.

## Acceptance criteria

- [ ] Applying a Copilot run creates one commit with Kind, Run-Id, Model and Tokens trailers
- [ ] Ship creates a commit and records the PR number in a trailer
- [ ] History timeline can be rebuilt from `git log` with no other state
- [ ] Commits are authored with the user's git identity (or GitHub noreply email if unset)

## Notes

Absorbs GH-006, which duplicated this item and was removed during backlog reconciliation.

Absorbs GH-024, which duplicated this item and was removed during backlog reconciliation.
