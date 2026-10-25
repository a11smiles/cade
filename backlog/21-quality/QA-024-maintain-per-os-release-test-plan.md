---
id: QA-024
title: Maintain a per-OS release test plan
tags: [qa, test]
epic: QA
priority: P0
depends_on: [QA-001]
---

## Description

A checklist run before every release on macOS (Apple silicon and Intel), Windows 10/11 and two Linux distros (X11 and Wayland): install, update, first run, each output type's primary flow, imports, ship, deploy plan, feedback submission and uninstall.

## Acceptance criteria

- [ ] Checklist lives in `docs/release-checklist.md`
- [ ] Each release records who ran it and the results
- [ ] Failed checks block the release
