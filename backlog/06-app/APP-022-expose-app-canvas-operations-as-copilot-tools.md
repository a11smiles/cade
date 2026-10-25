---
id: APP-022
title: Expose app canvas operations as Copilot tools
tags: [app, rust, feature]
epic: APP
priority: P0
depends_on: [COPILOT-020, APP-005]
design: design/exports/v0001-2026-10-25/Copilot.dc.html
---

## Description

Register App-specific agent tools: create frame from component list, apply variant/state, wire interaction, bind form to entity, run codegen, run DS lint, render frame to image for self-check. Tools write staged change sets, never direct commits.

## Acceptance criteria

- [ ] Each tool has a JSON schema and is listed in the agent's tool set for App projects
- [ ] Tool calls produce staged changes visible in the run screen
- [ ] Render-to-image returns a PNG the agent can inspect
- [ ] Tools refuse operations outside the project or against policy
