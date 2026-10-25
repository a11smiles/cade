---
id: COPILOT-010
title: Build CADE's own agent loop for BYOK providers
tags: [copilot, feature, rust]
epic: COPILOT
priority: P1
depends_on: [COPILOT-002, COPILOT-018, COPILOT-019]
---

## Description

Copilot provides its own planner and tool loop; BYOK providers only return completions. Implement an agent loop in `cade-copilot` that sends the conversation and CADE tool schemas, executes returned tool calls through the same permission and change-set path as Copilot, feeds results back, and stops on a final answer, a max-steps limit, cancellation or an error. It emits plan updates so the plan UI (COPILOT-025) works the same for every provider.

## Acceptance criteria

- [ ] A scripted fake provider run with 5 tool calls completes and produces one change set
- [ ] Max steps (default 25, configurable) stops runaway loops with a 'Run stopped: step limit' state
- [ ] Tool calls that need permission pause the loop until the user answers
- [ ] Behavior parity test: the same scripted scenario produces identical change sets via Copilot mock and BYOK loop
