---
id: CANVAS-004
title: Render text with Parley, including fallback, bidi and variable fonts
tags: [canvas, rust, feature, a11y]
epic: CANVAS
priority: P0
depends_on: [CANVAS-003]
---

## Description

Implement `cade-text` on Parley for shaping and line breaking. Support system and bundled fonts, per-run styles, font fallback chains for CJK and emoji, bidirectional text, variable font axes, letter/line spacing, and paragraph alignment. Text layers resize as auto-width, auto-height or fixed.

## Acceptance criteria

- [ ] Latin, Arabic, Hebrew, CJK and emoji render correctly in the same text layer
- [ ] Missing glyphs fall back through a configurable chain; the properties strip flags substituted fonts
- [ ] Variable font axes (weight, width, slant, custom) are editable and render live
- [ ] Auto-width, auto-height and fixed resizing behave like the design reference
