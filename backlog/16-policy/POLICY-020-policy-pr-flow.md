---
id: POLICY-020
title: Publish policy changes as a pull request
tags: [policy, feature, rust]
epic: POLICY
priority: P1
depends_on: [POLICY-019, GH-010]
design: design/exports/v0001-2026-10-25/Admin.dc.html
---

## Description

'Open policy PR' creates a branch on the org's `.github` repo, commits the updated `cade/policy.yml` with the version incremented, and opens a PR with a human-readable change summary. Reviewers come from CODEOWNERS. Clients pick up the change after merge (POLICY-005).

## Acceptance criteria

- [ ] PR contains only the policy file change and a summary
- [ ] `version` increments automatically
- [ ] Validation runs before the PR is created
- [ ] A ready-made GitHub Action (`cade policy validate`) is offered for the repo's CI
