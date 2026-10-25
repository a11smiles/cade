---
id: COPILOT-006
title: Add the OpenAI BYOK provider
tags: [copilot, feature, rust]
epic: COPILOT
priority: P1
depends_on: [COPILOT-002, COPILOT-009, COPILOT-010]
---

## Description

Implement `ModelProvider` for the OpenAI API with streaming and function/tool calling, keyed from the OS keychain. Handle parallel tool calls by executing them sequentially through the change-set engine so ordering stays deterministic.

## Acceptance criteria

- [ ] Agent run with tool calls completes end to end against the OpenAI API in a live test
- [ ] Parallel tool calls in one response are applied in the order returned
- [ ] Rate-limit (429) responses back off and retry up to 3 times with a visible 'Waiting for provider' status
- [ ] Token usage is recorded per run
- [ ] Hidden when BYOK is disabled by policy
