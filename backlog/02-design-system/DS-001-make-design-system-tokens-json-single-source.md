---
id: DS-001
title: Make design/system/tokens.json the single source of UI tokens
tags: [ds, feature, ui, ts, rust]
epic: DS
priority: P0
depends_on: [FOUND-002]
---

## Description

Store CADE's own UI tokens in W3C Design Tokens (DTCG) format at `design/system/tokens.json`, with `dark` and `light` values for every color. A build step generates CSS custom properties for `app/ui` and a Rust module in `cade-render` for colors drawn natively on the canvas (selection, guides, handles, Copilot cursor). No component may use a raw hex value.

## Acceptance criteria

- [ ] `just tokens` regenerates `tokens.css` and `cade-render/src/theme.rs`
- [ ] CI fails if generated files are stale
- [ ] A lint rule rejects raw hex colors in `app/ui/src` outside generated files
- [ ] Token names follow `color.surface.*`, `color.text.*`, `color.border.*`, `color.accent.*`, `color.status.*`
