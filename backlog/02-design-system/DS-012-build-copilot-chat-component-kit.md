---
id: DS-012
title: Build the Copilot chat component kit
tags: [ds, feature, ui]
epic: DS
priority: P0
depends_on: [DS-007, DS-008]
design: design/exports/v0001-2026-10-25/Copilot.dc.html
---

## Description

Components used by every Copilot panel: user and assistant messages, context chips, plan checklist with live step status, staged change list (ADD/MOD/DEL rows), question card with choices, permission prompt card, failure card, token usage meter, model picker and composer with @-mentions and slash commands.

## Acceptance criteria

- [ ] Each component has stories in both themes
- [ ] Streaming assistant text renders without layout jumps
- [ ] Permission and failure cards are announced via `role=status`
- [ ] Composer supports multi-line input, @layer, @file and / commands
