---
id: COPILOT-008
title: Add the OpenAI-compatible endpoint provider
tags: [copilot, feature, rust]
epic: COPILOT
priority: P1
depends_on: [COPILOT-006]
---

## Description

Allow any OpenAI-compatible endpoint (base URL, optional key, optional custom headers, model list fetched from `/models` or entered manually). This covers self-hosted gateways and local servers. Show a capability check on save: streaming, tool calling and context length.

## Acceptance criteria

- [ ] Adding an endpoint runs a capability probe and shows which of streaming/tools/vision are supported
- [ ] Endpoints without tool calling are allowed for chat only and agent features are disabled with an explanation
- [ ] Custom headers are stored in the keychain because they often carry credentials
- [ ] Policy `models.byok_endpoints` allowlist (when set) blocks non-listed hosts
