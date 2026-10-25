---
id: GH-001
title: Sign in with GitHub using the OAuth device flow
tags: [gh, feature, rust, security]
epic: GH
priority: P0
depends_on: []
design: design/exports/v0001-2026-10-25/Onboarding.dc.html
---

## Description

Authenticate with GitHub through the OAuth device flow from a GitHub App owned by the project, opening the browser for the user's SSO. One sign-in serves Copilot, git operations, PRs and feedback issues.

## Acceptance criteria

- [ ] Sign-in opens the browser and completes without copying tokens manually
- [ ] Requested permissions are the minimum for repos, PRs, issues and Pages
- [ ] Org SSO authorization is detected and the user is prompted when needed
- [ ] Sign-out revokes and deletes the token
