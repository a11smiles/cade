---
id: APP-015
title: Build Prototype mode: flows
tags: [app, ts, rust, feature]
epic: APP
priority: P0
depends_on: [APP-002]
design: design/exports/v0001-2026-10-25/Prototype.dc.html
---

## Description

Prototype mode lists flows (e.g. Happy path, Missing info, Save & exit) with start screens. Flow arrows connect frames on the canvas with labeled triggers. Copilot-suggested flows show as dashed until accepted.

## Acceptance criteria

- [ ] Users can create, rename, reorder and delete flows with a start frame
- [ ] Connections render as arrows between frames with trigger labels
- [ ] Suggested flows from Copilot are visually distinct until accepted
