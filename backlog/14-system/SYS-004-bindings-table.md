---
id: SYS-004
title: Show the bindings table with status and fixes
tags: [sys, feature, ui]
epic: SYS
priority: P0
depends_on: [SYS-002]
design: design/exports/v0001-2026-10-25/Deploy.dc.html
---

## Description

Table of from, binding, to and status, with inline fix actions (e.g. Add to IaC) that stage changes in the right workspace.

## Acceptance criteria

- [ ] Every binding appears with its status
- [ ] Broken bindings show a fix action when one exists
- [ ] Fix actions stage changes, not apply them
