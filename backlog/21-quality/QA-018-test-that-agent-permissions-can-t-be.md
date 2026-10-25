---
id: QA-018
title: Test that agent permissions can't be bypassed
tags: [qa, test, security, policy]
epic: QA
priority: P0
depends_on: [QA-016, CORE-011, POLICY-008, COPILOT-029]
---

## Description

Adversarial tests where the model is scripted to attempt disallowed actions: shell without approval, writes outside the project, network with policy off, unapproved packages, edits that bypass the operations API, and policy file modification. Every attempt must be blocked and logged.

## Acceptance criteria

- [ ] A suite of at least 25 bypass attempts runs in CI
- [ ] Every attempt is blocked and produces an audit entry
- [ ] New agent tools must add tests to the suite before merge
