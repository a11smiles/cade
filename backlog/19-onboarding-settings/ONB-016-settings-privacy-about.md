---
id: ONB-016
title: Implement Settings › Privacy and About
tags: [onb, feature, ui, security]
epic: ONB
priority: P1
depends_on: [ONB-008, POLICY-015, POLICY-023]
design: design/exports/v0001-2026-10-25/Settings.dc.html
---

## Description

Privacy shows what CADE sends and to whom (Copilot requests to GitHub, BYOK provider requests, optional reporting) and links to the audit log. About shows app, Copilot CLI, renderer and policy versions, license and open-source notices, and 'Reveal logs'.

## Acceptance criteria

- [ ] Privacy page lists every outbound destination currently enabled
- [ ] About shows versions and a copy-to-clipboard diagnostics block
- [ ] Open-source notices list all bundled dependencies
- [ ] Reveal logs opens the log folder
