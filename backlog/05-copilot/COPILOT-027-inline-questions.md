---
id: COPILOT-027
title: Ask clarifying questions inline with options
tags: [copilot, feature, ui, rust]
epic: COPILOT
priority: P1
depends_on: [COPILOT-014, COPILOT-018]
design: design/exports/v0001-2026-10-25/Main.dc.html
---

## Description

Add an `ask_user` tool: Copilot pauses and shows a question card with 2–4 option buttons and free text (e.g. 'Camera sheet / File picker / Both'). The run resumes with the answer as the tool result. Questions are kept in the thread for later reference.

## Acceptance criteria

- [ ] Calling `ask_user` pauses the run and shows the card
- [ ] Clicking an option resumes the run with that answer
- [ ] Free-text answer is supported
- [ ] Unanswered questions time out only when the user stops the run, never automatically
