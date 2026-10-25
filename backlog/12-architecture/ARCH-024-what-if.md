---
id: ARCH-024
title: Run what-if against a chosen subscription
tags: [arch, feature, rust]
epic: ARCH
priority: P1
depends_on: [ARCH-023, ARCH-013]
design: design/exports/v0001-2026-10-25/Architecture.dc.html
---

## Description

Run a deployment what-if for the selected environment against its subscription and resource group, and show creates, modifies and deletes on the canvas and in a summary. Never performs a deployment.

## Acceptance criteria

- [ ] What-if results color tiles by create, modify, delete and no change
- [ ] Property-level changes are listed per resource
- [ ] Running what-if requires an explicit user action and shows the target subscription
- [ ] Errors from Azure are shown with the failing resource selected
