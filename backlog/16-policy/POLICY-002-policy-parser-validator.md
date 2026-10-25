---
id: POLICY-002
title: Parse and validate policy files with clear errors
tags: [policy, feature, rust]
epic: POLICY
priority: P0
depends_on: [POLICY-001]
---

## Description

Implement parsing (serde_yaml) and validation in `cade-policy` with line/column error messages. Invalid policies are rejected; CADE keeps the last valid cached policy and shows the admin-facing error in Settings › Org policy.

## Acceptance criteria

- [ ] Syntax errors report file, line and column
- [ ] Semantic errors (e.g. preapproved command while shell is deny) are flagged
- [ ] An invalid new policy never replaces a valid cached one
- [ ] `cade policy validate <file>` CLI subcommand exits non-zero on invalid files for use in CI
