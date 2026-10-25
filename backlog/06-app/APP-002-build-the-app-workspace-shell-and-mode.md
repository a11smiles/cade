---
id: APP-002
title: Build the App workspace shell and mode switch
tags: [app, ts, ui, feature]
epic: APP
priority: P0
depends_on: [APP-001, DS-010]
design: design/exports/v0001-2026-10-25/Main.dc.html
---

## Description

Header with APP tag, project name and path, the Design / Prototype / Code mode switch, policy pill, zoom, version history, Deploy app, and Ship to repo. Switching modes keeps selection and viewport where they apply.

## Acceptance criteria

- [ ] Header matches the design reference in dark and light themes
- [ ] Mode switch changes the workspace without reloading the project
- [ ] Selection persists across Design, Prototype and Code modes
- [ ] The policy pill shows the active org policy and opens its details
