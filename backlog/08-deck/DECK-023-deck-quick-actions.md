---
id: DECK-023
title: Add deck quick actions in the Copilot column
tags: [deck, feature, ui]
epic: DECK
priority: P2
depends_on: [DECK-022]
design: design/exports/v0001-2026-10-25/Slides.dc.html
---

## Description

Contextual quick actions: tighten every title to one line, write speaker notes for all slides, apply a
master, turn a doc or app flow into a deck outline. Actions are prompts with structured parameters, so they
are deterministic to test.

## Acceptance criteria

- [ ] Quick actions appear only when relevant to the current selection or deck
- [ ] Each action shows its staged changes before applying
- [ ] Actions are defined in a registry so new ones need no UI changes
