---
id: APP-007
title: Embed a code editor in Code mode
tags: [app, ts, ui, feature]
epic: APP
priority: P0
depends_on: [APP-006]
design: design/exports/v0001-2026-10-25/Code.dc.html
---

## Description

Code mode shows a file tree, tabbed editor (Monaco or CodeMirror 6) with TypeScript language service, highlighting of the code range for the canvas selection, and a status bar for tsc, ESLint and DS lint.

## Acceptance criteria

- [ ] File tree shows the project's source files with modified markers
- [ ] Selecting a layer highlights its code range in the editor
- [ ] TypeScript errors and lint results appear inline and in the status bar
- [ ] Files generated from the data model are labeled 'from model'
