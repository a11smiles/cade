---
id: POLICY-025
title: Add conformance tests for every policy rule
tags: [policy, test, rust]
epic: POLICY
priority: P1
depends_on: [POLICY-007, POLICY-008, POLICY-015]
---

## Description

Create a conformance suite: for each rule and value, a test proves the enforcement point blocks or allows as specified, including attempts to bypass via the UI, the agent tools and the CLI. Runs in CI on all three OSes.

## Acceptance criteria

- [ ] Every schema key has at least one allow and one deny test
- [ ] Bypass attempts through agent tools are covered
- [ ] Suite runs on macOS, Windows and Linux
- [ ] Coverage report lists any rule without a test as a CI failure
