---
id: HIST-015
title: Keep history fast for large and old projects
tags: [hist, history, chore, rust, perf]
epic: HIST
priority: P2
depends_on: [HIST-003, HIST-004]
---

## Description

Index commits with trailers in a local cache so the timeline doesn't walk git on every open, render thumbnails lazily and cache them, and optionally squash autosave versions older than 90 days (user setting, never by default).

## Acceptance criteria

- [ ] Timeline for 10k commits opens in under 500 ms after first index
- [ ] Thumbnails are cached and invalidated by SHA
- [ ] Squash setting is off by default and shows exactly what it will merge
- [ ] Index rebuilds automatically if git history changes externally
