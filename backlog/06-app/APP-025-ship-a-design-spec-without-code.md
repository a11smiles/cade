---
id: APP-025
title: Ship a design spec without code
tags: [app, rust, feature]
epic: APP
priority: P1
depends_on: [APP-024]
design: design/exports/v0001-2026-10-25/Ship.dc.html
---

## Description

'Design spec only' ships frames, token usage and redlines as Markdown + images to the repo for engineers who build by hand.

## Acceptance criteria

- [ ] Spec output includes each frame as PNG, a token table and measurements
- [ ] Spec is readable on github.com without CADE
- [ ] Spec updates on re-ship rather than duplicating files
