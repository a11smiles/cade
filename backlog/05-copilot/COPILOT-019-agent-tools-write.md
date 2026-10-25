---
id: COPILOT-019
title: Expose write tools that produce staged operations
tags: [copilot, feature, rust]
epic: COPILOT
priority: P0
depends_on: [COPILOT-018, COPILOT-021, CORE-011]
---

## Description

Define write tools: `create_layer`, `update_layer(props)`, `delete_layer`, `move_layer`, `instantiate_component`, `set_variant`, `bind_token`, `create_frame`, `set_text`, `create_page`. Every write goes into the run's staged change set (COPILOT-021) on a Loro fork, never directly into the user's document. Tools validate against the design-system-only rule when policy requires it.

## Acceptance criteria

- [ ] Write tool calls never modify the main document until the user applies (test asserts main doc hash unchanged)
- [ ] Using a raw hex color when `design_systems` policy is on returns a tool error suggesting the nearest token
- [ ] Each successful call returns the new/changed layer ids
- [ ] Invalid ids or props return structured errors the model can recover from
