---
id: COPILOT-038
title: Keep a local, redacted run log for debugging
tags: [copilot, chore, rust, security]
epic: COPILOT
priority: P1
depends_on: [COPILOT-002]
---

## Description

Write a per-run debug log (events, tool names, timings, errors) to the app's log directory with prompt and document content redacted by default. A 'Include content' toggle exists for reproducing bugs locally. Logs rotate at 50 MB.

## Acceptance criteria

- [ ] Logs contain tool names and timings but no prompt text by default
- [ ] Toggle 'Include content' adds prompts to new logs only
- [ ] Logs rotate at 50 MB and keep 5 files
- [ ] 'Reveal logs' in Settings › About opens the folder
