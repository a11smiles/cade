---
id: IMPORT-004
title: Discover exported Spark repos with the GitHub API
tags: [import, feature, rust]
epic: IMPORT
priority: P0
depends_on: [IMPORT-001, GH-001]
design: design/exports/v0001-2026-10-25/Import.dc.html
---

## Description

List repos the signed-in user can read (personal and org) and detect Spark exports by markers such as
the Spark runtime package in `package.json`, Spark config files and `useKV`/`llm()` usage. Show language,
file count and detected features (uses llm(), KV store). Allow adding a repo by URL.

## Acceptance criteria

- [ ] Exported Spark repos the user can read are listed within 5 seconds for an account with 200 repos
- [ ] Detection reports uses-llm and uses-KV flags correctly on reference repos
- [ ] Adding a repo by URL validates access and detection
- [ ] Respects GitHub API rate limits with backoff
