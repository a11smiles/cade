---
id: APP-008
title: Generate responsive CSS from breakpoints
tags: [app, rust, feature]
epic: APP
priority: P1
depends_on: [CANVAS-023, APP-005]
---

## Description

Emit media queries (or container queries) from canvas breakpoints and their overrides so one component works across phone, tablet and desktop.

## Acceptance criteria

- [ ] Each breakpoint produces a matching media query
- [ ] Overrides only appear in the breakpoint where they're set
- [ ] Generated output renders the same as the canvas at each breakpoint width
