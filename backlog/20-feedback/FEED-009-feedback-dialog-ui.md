---
id: FEED-009
title: Build the in-app Send feedback dialog
tags: [feed, feedback, feature, ui, a11y]
epic: FEED
priority: P0
depends_on: [DS-015, FEED-007]
design: design/exports/v0001-2026-10-25/Feedback.dc.html
---

## Description

Help › Send feedback (and a feedback button in the header) opens a dialog with three types — Bug, Feature request, Comment — each showing fields that mirror its issue form, prefilled output type and area from the current workspace, and the signed-in identity ('Posting as @login to <owner>/cade').

## Acceptance criteria

- [ ] Switching type changes fields without losing shared fields
- [ ] Output type is prefilled from the current workspace
- [ ] Required fields are enforced with inline errors
- [ ] Dialog is fully keyboard-operable and screen-reader labelled
- [ ] Draft persists if the dialog is closed
