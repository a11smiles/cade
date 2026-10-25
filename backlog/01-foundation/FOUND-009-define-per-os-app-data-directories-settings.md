---
id: FOUND-009
title: Define per-OS app data directories and settings migrations
tags: [found, feature, rust]
epic: FOUND
priority: P0
depends_on: [FOUND-001]
---

## Description

Use the `directories` crate to place config, cache, logs and local state in the right OS locations. App settings are a versioned TOML file with forward migrations; a newer file opened by an older build is backed up and not overwritten.

## Acceptance criteria

- [ ] Paths match OS conventions and are listed in `docs/paths.md`
- [ ] Settings schema has a version number and migration tests from every prior version
- [ ] An unknown future version is preserved and the app falls back to defaults read-only
- [ ] Cache can be cleared without touching settings or projects
