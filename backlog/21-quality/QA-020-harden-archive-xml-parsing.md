---
id: QA-020
title: Harden archive and XML parsing
tags: [qa, feature, security, rust]
epic: QA
priority: P0
depends_on: [QA-019]
---

## Description

All Office Open XML and zip inputs go through one hardened reader: reject zip-slip paths, cap entry count and total uncompressed size (zip bombs), disable DTDs and external entities (XXE, billion laughs), and enforce per-file time and memory limits.

## Acceptance criteria

- [ ] Known malicious samples (zip-slip, zip bomb, XXE, billion laughs) are rejected in tests
- [ ] Limits are configurable and documented
- [ ] Rejections produce a clear user error, not a crash
