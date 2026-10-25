---
id: HIST-004
title: Load any version as a read-only document
tags: [hist, history, feature, rust]
epic: HIST
priority: P0
depends_on: [HIST-001, CORE-015]
---

## Description

Check out the `.cade` snapshot at a commit into memory (without touching the working tree) and open it as a read-only Loro document for rendering and diffing.

## Acceptance criteria

- [ ] Loading a version does not modify the working tree or index
- [ ] Read-only document rejects edits with a clear message
- [ ] Loading a 50 MB project version takes under 1 s on reference hardware
- [ ] Versions from older .cade format versions migrate in memory (CORE migration path)
