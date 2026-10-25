---
id: GH-005
title: Wrap git operations with gix
tags: [gh, feature, rust]
epic: GH
priority: P0
depends_on: []
---

## Description

Implement `cade-git` on gix (with git2 fallback where gix lacks a feature) for init, clone, status, commit, branch, checkout, fetch, push and merge-base, authenticated with the GitHub token.

## Acceptance criteria

- [ ] Clone, commit, branch and push work against GitHub.com and GHES in integration tests
- [ ] Operations run off the UI thread with progress reporting
- [ ] Large binary assets are stored with Git LFS when present
- [ ] Credentials are never written to git config
