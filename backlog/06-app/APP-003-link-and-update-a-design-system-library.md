---
id: APP-003
title: Link and update a design system library
tags: [app, rust, ts, feature]
epic: APP
priority: P0
depends_on: [CORE-008, CORE-007]
design: design/exports/v0001-2026-10-25/Main.dc.html
---

## Description

Projects link one or more design system libraries (from the org or local). The left panel card shows name, version, publisher and component/token counts. When a new version is published, show an update prompt with a change summary.

## Acceptance criteria

- [ ] The linked library card shows name, version, publisher and counts
- [ ] Updating to a new version updates instances and tokens and is one undo step
- [ ] A change summary lists added, changed and removed components before updating
- [ ] Policy can restrict projects to approved libraries only (POLICY epic)
