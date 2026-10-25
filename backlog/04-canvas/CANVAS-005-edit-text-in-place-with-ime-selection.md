---
id: CANVAS-005
title: Edit text in place with IME, selection and caret
tags: [canvas, rust, ts, feature, a11y]
epic: CANVAS
priority: P0
depends_on: [CANVAS-004, CANVAS-002]
---

## Description

Double-click a text layer to edit in place on the canvas. Support caret navigation, word/line selection, IME composition (macOS, Windows TSF, IBus/Fcitx), spellcheck underlines from the OS, and rich-run edits (bold a word) that write to the CRDT text type.

## Acceptance criteria

- [ ] IME composition works for Japanese, Chinese and Korean input on all three OSes
- [ ] Arrow, word, line and document navigation match platform conventions
- [ ] Applying a style to a selection splits runs correctly and is one undo step
- [ ] Concurrent edits to the same text layer from two replicas merge without lost characters
