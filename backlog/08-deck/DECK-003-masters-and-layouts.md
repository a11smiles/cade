---
id: DECK-003
title: Implement slide masters and layouts with inheritance
tags: [deck, feature, rust]
epic: DECK
priority: P0
depends_on: [DECK-001, CORE-003]
design: design/exports/v0001-2026-10-25/Slides.dc.html
---

## Description

A master defines background, theme tokens and decorative layers; layouts belong to a master and define
named placeholders. Slides inherit from their layout and master: inherited layers render but are not
editable on the slide unless detached. Editing a master or layout updates every slide that uses it.
Masters can come from the org design system (e.g. "Contoso Board") or from an imported .potx.

## Acceptance criteria

- [ ] Changing a master background token updates all slides using that master in one undoable step
- [ ] A slide can switch layout and keeps content that maps to placeholders of the same kind
- [ ] Inherited layers are visually marked and locked; 'Detach from layout' makes a local copy
- [ ] Masters published in a design system appear in the master picker in the header
- [ ] A deck can mix slides from two masters
