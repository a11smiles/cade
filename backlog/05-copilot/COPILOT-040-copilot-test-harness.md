---
id: COPILOT-040
title: Build a recorded-session test harness for Copilot flows
tags: [copilot, test, rust]
epic: COPILOT
priority: P1
depends_on: [COPILOT-002, COPILOT-021]
---

## Description

Create a harness that replays recorded event streams (from Copilot and BYOK providers) against the agent loop, change-set engine and UI state store, so regressions in tool handling, staging and failure states are caught without network access.

## Acceptance criteria

- [ ] At least 10 recorded scenarios covering success, policy block, failure mid-run, offline, model fallback
- [ ] Harness runs in under 60 s in CI on all three OSes
- [ ] Scenario files are human-readable JSONL
- [ ] A new scenario can be recorded from a live run with a CLI flag
