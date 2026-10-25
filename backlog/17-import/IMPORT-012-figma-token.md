---
id: IMPORT-012
title: Manage the user's Figma access token
tags: [import, feature, rust, security]
epic: IMPORT
priority: P0
depends_on: [IMPORT-001, FOUND-010]
design: design/exports/v0001-2026-10-25/Import.dc.html
---

## Description

Prompt for a Figma personal access token (or OAuth where available), validate it, and store it in the OS
keychain. Never write it to project files or logs. Show which scopes are needed (file read, library read,
variables read).

## Acceptance criteria

- [ ] Token is stored only in the OS keychain on all three platforms
- [ ] Invalid or expired tokens show a clear error and a re-enter action
- [ ] Logs and crash reports redact the token
- [ ] Token can be removed from Settings
