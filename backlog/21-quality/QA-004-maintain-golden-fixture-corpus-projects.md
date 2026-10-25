---
id: QA-004
title: Maintain a golden fixture corpus of projects
tags: [qa, test, rust]
epic: QA
priority: P0
depends_on: [CORE-015]
---

## Description

Keep a corpus of `.cade` projects covering every node kind, every output type, every format version and known tricky cases (nested instances, deep overrides, token cycles, missing assets). Each release adds a fixture saved by that version so backward compatibility is always tested.

## Acceptance criteria

- [ ] Every format version has at least one fixture
- [ ] Fixtures open, migrate, save and re-open without diff in CI
- [ ] Adding a fixture is documented in `docs/testing.md`
