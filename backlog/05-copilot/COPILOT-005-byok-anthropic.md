---
id: COPILOT-005
title: Add the Anthropic BYOK provider
tags: [copilot, feature, rust]
epic: COPILOT
priority: P1
depends_on: [COPILOT-002, COPILOT-009, COPILOT-010]
---

## Description

Implement `ModelProvider` for the Anthropic Messages API with streaming and tool use, using an API key stored in the OS keychain. Map Anthropic tool_use/tool_result blocks onto CADE's tool events and report input, output and cache token usage. Only available when policy allows BYOK (POLICY-007).

## Acceptance criteria

- [ ] A user with a valid key can complete a multi-turn agent run that calls at least two CADE tools
- [ ] Streaming text appears incrementally in the Copilot panel
- [ ] Invalid key produces the Auth error with a 'Check your key' action, not a raw HTTP error
- [ ] Token usage including cache read/write is recorded per run
- [ ] Provider is hidden and unselectable when `models.byok: false` in policy
