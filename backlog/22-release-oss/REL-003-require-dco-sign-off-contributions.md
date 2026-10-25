---
id: REL-003
title: Require DCO sign-off on contributions
tags: [rel, chore]
epic: REL
priority: P0
depends_on: [REL-001]
---

## Description

Use the Developer Certificate of Origin rather than a CLA so ownership stays clear without paperwork. Enforce `Signed-off-by` on every commit with a DCO check on PRs.

## Acceptance criteria

- [ ] DCO check runs on every PR
- [ ] CONTRIBUTING explains how to sign off and fix missing sign-offs
- [ ] Maintainer commits are held to the same check
