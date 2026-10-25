---
id: COPILOT-003
title: Implement the Copilot provider on the SDK
tags: [copilot, feature, rust]
epic: COPILOT
priority: P0
depends_on: [COPILOT-002, GH-001]
---

## Description

Implement `ModelProvider` for GitHub Copilot using the SDK session API, authenticated with the signed-in user's GitHub token. Register CADE's agent tools (COPILOT-018/019/020) with the SDK session so Copilot's own planner can call them, and translate SDK events into `AgentEvent`s. Respect the SDK's model list so only models the user's Copilot plan and org allow are offered.

## Acceptance criteria

- [ ] Signed-in user can send a prompt and receive a streamed answer through the Copilot provider
- [ ] CADE tools registered on the session are callable by Copilot and their results are returned to the session
- [ ] `list_models()` returns the SDK's runtime model list, not a hard-coded one
- [ ] Token usage reported by the SDK is surfaced as `AgentEvent::Usage`
- [ ] SDK default of all first-party tools enabled is overridden: only tools allowed by CADE policy (COPILOT-028, POLICY-008) are enabled

## Notes

The SDK defaults to the equivalent of `--allow-all`. CADE must pass an explicit tool allowlist on every session.
