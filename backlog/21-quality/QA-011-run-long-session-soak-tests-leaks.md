---
id: QA-011
title: Run long-session soak tests for leaks
tags: [qa, test, perf]
epic: QA
priority: P1
depends_on: [QA-007]
---

## Description

An 8-hour scripted session that edits, applies Copilot runs, switches workspaces and exports, while sampling memory, GPU memory, handles and file descriptors.

## Acceptance criteria

- [ ] Memory growth stays under 10% after warm-up over 8 hours
- [ ] No file descriptor or handle leaks
- [ ] Soak runs weekly on all three OSes
