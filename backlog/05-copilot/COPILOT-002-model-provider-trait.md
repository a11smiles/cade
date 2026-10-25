---
id: COPILOT-002
title: Define the ModelProvider trait and streaming event types
tags: [copilot, feature, rust]
epic: COPILOT
priority: P0
depends_on: [COPILOT-001]
---

## Description

Define a `ModelProvider` trait in `cade-copilot` so Copilot and every BYOK provider are interchangeable. The trait covers listing models, starting an agent turn with messages + tool definitions, streaming `AgentEvent`s (text delta, tool call request, tool result ack, usage, plan update, done, error) and cancelling. Copilot runs through the SDK's own agent loop; BYOK providers run through a CADE-owned agent loop (COPILOT-010) that emits the same events, so the UI cannot tell them apart.

## Acceptance criteria

- [ ] Trait and event enums compile with no provider-specific types in their signatures
- [ ] A `FakeProvider` test double can script a sequence of events and is used by UI and change-set tests
- [ ] Usage events carry input, output and cached token counts separately
- [ ] Cancellation stops event delivery within 500 ms in a test with a slow fake provider
- [ ] The provider in use for a session is recorded on every run for history and metering
