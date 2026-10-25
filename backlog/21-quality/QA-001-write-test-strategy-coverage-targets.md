---
id: QA-001
title: Write the test strategy and coverage targets
tags: [qa, docs]
epic: QA
priority: P0
depends_on: [FOUND-001]
---

## Description

Document which layers are tested how: unit and property tests in Rust crates, component tests in `app/ui`, IPC integration tests, end-to-end tests of key flows, renderer golden images, fuzzing for importers and performance benchmarks. Set coverage targets (80% line coverage for `cade-core`, `cade-crdt`, `cade-policy`, importers; 60% elsewhere) and which suites gate PRs versus nightly.

## Acceptance criteria

- [ ] `docs/testing.md` lists every suite, where it runs and what blocks merge
- [ ] Coverage is measured with `cargo llvm-cov` and Vitest and reported on PRs
- [ ] Targets per crate are enforced in CI
