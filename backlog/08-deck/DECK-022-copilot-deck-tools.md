---
id: DECK-022
title: Expose deck operations as Copilot agent tools
tags: [deck, feature, rust]
epic: DECK
priority: P0
depends_on: [COPILOT-020, DECK-007, DECK-016]
design: design/exports/v0001-2026-10-25/Slides.dc.html
---

## Description

Register agent tools for deck work: create slide from layout, set placeholder text, insert linked frame,
set chart kind/data/series token, write speaker notes, apply master, reorder slides. All tool calls produce
staged change sets that the user reviews, and edits are scoped to the current selection when one exists.

## Acceptance criteria

- [ ] 'Make the pilot quarters stand out' on a selected chart changes only that chart
- [ ] Every tool call appears as a reviewable change (SERIES, DEL, LABELS rows) before applying
- [ ] Copilot leaves placeholders for numbers not in the sources and says so
- [ ] Tools are covered by integration tests with a mocked model
