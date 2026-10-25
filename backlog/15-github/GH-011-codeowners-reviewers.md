---
id: GH-011
title: Request reviewers from CODEOWNERS
tags: [gh, feature, rust]
epic: GH
priority: P1
depends_on: [GH-010]
design: design/exports/v0001-2026-10-25/Ship.dc.html
---

## Description

Parse the target repo's CODEOWNERS for the changed paths and show the teams/users who will be requested, allowing extra reviewers.

## Acceptance criteria

- [ ] Reviewers shown match GitHub's CODEOWNERS resolution for the changed files
- [ ] Extra reviewers can be added
- [ ] Missing CODEOWNERS shows no reviewers without error
