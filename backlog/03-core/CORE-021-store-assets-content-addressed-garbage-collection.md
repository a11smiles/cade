---
id: CORE-021
title: Store assets content-addressed with garbage collection
tags: [core, feature, rust]
epic: CORE
priority: P0
depends_on: [CORE-015]
---

## Description

Images, fonts, videos and imported binaries are stored once by SHA-256 in `assets/`. References come from nodes. A GC pass removes unreferenced assets older than the newest commit that references them. Optional git LFS tracking for files over a size threshold.

## Acceptance criteria

- [ ] Inserting the same image twice stores one blob
- [ ] GC never deletes an asset referenced by any commit on the current branch
- [ ] Assets over 10 MB are tracked by LFS when enabled
- [ ] Missing asset files show a placeholder and a repair action
