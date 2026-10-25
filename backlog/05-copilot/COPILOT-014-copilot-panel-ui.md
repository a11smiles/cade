---
id: COPILOT-014
title: Build the Copilot panel message list and composer
tags: [copilot, feature, ui, ts, a11y]
epic: COPILOT
priority: P0
depends_on: [COPILOT-002, DS-012]
design: design/exports/v0001-2026-10-25/Main.dc.html
---

## Description

Build the docked Copilot column used by every workspace: header with thread name and expand control, streaming message list with Markdown rendering and code blocks, run cards (plan, staged changes, questions, permission prompts), and the composer with context chips, mode picker (Agent/Ask), model picker, send and stop. Expand opens the wide agent-run layout from Copilot.dc.html.

## Acceptance criteria

- [ ] Streaming text renders incrementally without layout jank at 60 fps
- [ ] Composer supports multiline input, Enter to send, Shift+Enter for newline, Esc to stop a run
- [ ] All controls are reachable by keyboard and announced correctly by VoiceOver and NVDA
- [ ] Expanding switches to the split agent-run view and back without losing scroll position
- [ ] Panel width is resizable between 320 and 640 px and persists
