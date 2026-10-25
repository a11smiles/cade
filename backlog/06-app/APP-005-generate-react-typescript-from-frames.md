---
id: APP-005
title: Generate React + TypeScript from frames
tags: [app, rust, feature]
epic: APP
priority: P0
depends_on: [APP-004, CANVAS-021]
design: design/exports/v0001-2026-10-25/Code.dc.html
---

## Description

`cade-codegen-react` turns frames into React components using the mapped design system components, Taffy layout as flex/grid CSS, token references instead of raw values, and accessible markup. Output is idiomatic, formatted with Prettier, and stable across runs.

## Acceptance criteria

- [ ] Generated code compiles with `tsc` and passes ESLint with the project config
- [ ] Layout output matches the canvas within 1 px for fixture frames
- [ ] No raw color, spacing or type values appear when tokens exist (DS lint passes)
- [ ] Re-running codegen on an unchanged frame produces an identical file
