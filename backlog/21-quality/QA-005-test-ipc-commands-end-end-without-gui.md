---
id: QA-005
title: Test IPC commands end to end without a GUI
tags: [qa, test, rust]
epic: QA
priority: P0
depends_on: [FOUND-004]
---

## Description

Use Tauri's mock runtime to invoke every command with valid and invalid payloads and assert results, emitted events and error codes. Commands that touch the network use recorded fixtures.

## Acceptance criteria

- [ ] Every registered command has at least one success and one failure test
- [ ] Error codes match the documented error model
- [ ] Tests run headless on all three OSes in CI
