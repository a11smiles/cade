---
id: DS-017
title: Theme the code editor and diff views
tags: [ds, feature, ui]
epic: DS
priority: P0
depends_on: [DS-002, DS-004]
design: design/exports/v0001-2026-10-25/Code.dc.html
---

## Description

Use CodeMirror 6 for Code mode, IaC and DDL drawers, with a CADE syntax theme generated from tokens for dark and light. Provide inline diff highlighting (added/removed lines) and a "linked to selection" highlight band.

## Acceptance criteria

- [ ] Syntax theme covers TS/TSX, Bicep, HCL, SQL, YAML and JSON
- [ ] Diff lines and selection band are visible in both themes
- [ ] Editor respects the app font and size tokens
