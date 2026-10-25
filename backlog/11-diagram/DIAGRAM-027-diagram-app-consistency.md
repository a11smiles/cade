---
id: DIAGRAM-027
title: Compare a process diagram with the App prototype
tags: [diagram, feature, rust]
epic: DIAGRAM
priority: P2
depends_on: [DIAGRAM-026, APP-015]
design: design/exports/v0001-2026-10-25/Diagram.dc.html
---

## Description

Copilot check that maps diagram steps to App prototype screens and validation rules and reports
mismatches (e.g. 'the app also checks the incident date; that step isn't in this diagram').

## Acceptance criteria

- [ ] Mismatches list the diagram step and the App screen or rule involved
- [ ] Each mismatch offers a staged fix to the diagram
- [ ] The check uses only project content as context
