---
id: IMPORT-010
title: Rewire Spark llm() calls to the Copilot SDK
tags: [import, feature, rust]
epic: IMPORT
priority: P0
depends_on: [IMPORT-006, COPILOT-001]
design: design/exports/v0001-2026-10-25/Import.dc.html
---

## Description

Codemod that replaces Spark `llm()` and prompt helpers (which used GitHub Models, now retired) with calls
through a generated client for the Copilot SDK or the user's BYOK provider, preserving prompt text and
response parsing. Lists every rewritten call in the report.

## Acceptance criteria

- [ ] Every llm() call site is rewritten and listed
- [ ] Rewritten calls compile and return responses in the preview
- [ ] The model provider is configurable (Copilot or BYOK)
- [ ] No API keys are written into source files
