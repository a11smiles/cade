---
id: QA-022
title: Verify secrets never leak into logs, exports or reports
tags: [qa, test, security]
epic: QA
priority: P0
depends_on: [FOUND-010, FOUND-014]
---

## Description

Seed known fake secrets (API keys, tokens) into settings, keychain and documents, then run flows and scan logs, crash reports, diagnostics bundles, JSON exports, commits and feedback submissions for them.

## Acceptance criteria

- [ ] No seeded secret appears in any artifact
- [ ] Scanner runs in CI on every PR
- [ ] GitHub secret scanning push protection is enabled on the repo
