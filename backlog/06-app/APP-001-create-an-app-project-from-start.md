---
id: APP-001
title: Create an App project from Start
tags: [app, ts, rust, feature]
epic: APP
priority: P0
depends_on: [CORE-003, FOUND-003]
design: design/exports/v0001-2026-10-25/Start.dc.html
---

## Description

Choosing App on the Start screen (or submitting the prompt with App selected) creates a project folder under the projects folder, initializes git, links the org's default design system, and opens the App workspace in Design mode. If a prompt was entered, it starts the first Copilot run.

## Acceptance criteria

- [ ] Selecting the App card creates a project folder with a git repo and opens Design mode
- [ ] The project links the org's approved design system by default
- [ ] A prompt entered on Start becomes the first message in the project's Copilot thread
- [ ] Attached PRDs, sketches or screenshots are copied into the project and added to Copilot context
