---
id: REL-016
title: Version releases and generate changelogs
tags: [rel, chore]
epic: REL
priority: P0
depends_on: [REL-005]
---

## Description

Use SemVer for the app and separately for the .cade format and policy schema. Generate `CHANGELOG.md` from Conventional Commits with git-cliff, grouped by surface (App, Image, Deck, Architecture…).

## Acceptance criteria

- [ ] CHANGELOG is generated on release
- [ ] Format and policy schema versions are listed in each release's notes
- [ ] Breaking changes are called out at the top of release notes
