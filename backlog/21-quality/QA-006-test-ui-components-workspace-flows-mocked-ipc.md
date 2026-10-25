---
id: QA-006
title: Test UI components and workspace flows with mocked IPC
tags: [qa, test, ts, ui]
epic: QA
priority: P0
depends_on: [FOUND-002, FOUND-004]
---

## Description

Vitest and Testing Library tests for components and per-workspace flows (properties strip edits, Copilot composer, staged change review, failure states) against a typed IPC mock generated from the bindings.

## Acceptance criteria

- [ ] IPC mock is generated from `bindings.ts` so it can't drift
- [ ] Each workspace has flow tests for its primary actions
- [ ] Failure states from the States screen each have a test

## Notes

Design reference: design/exports/v0001-2026-10-25/States.dc.html
