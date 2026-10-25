---
id: FOUND-012
title: Confine file access to project folders and watch for changes
tags: [found, feature, rust, security]
epic: FOUND
priority: P0
depends_on: [FOUND-001]
---

## Description

All file IO goes through a `ProjectFs` that canonicalizes paths and rejects anything outside the open project root (symlink and `..` escapes included). Use `notify` to watch the project for external edits (git pulls, editors) and raise change events the document layer can reconcile.

## Acceptance criteria

- [ ] Paths escaping the project root are rejected, including via symlinks
- [ ] External file edits produce a change event within 1 s
- [ ] Watchers are debounced so a git checkout of 1,000 files emits one batched event
- [ ] Works on case-insensitive (macOS, Windows) and case-sensitive (Linux) filesystems
