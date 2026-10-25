---
id: ONB-011
title: Implement Settings › Agent permissions
tags: [onb, feature, ui, security]
epic: ONB
priority: P0
depends_on: [ONB-008, COPILOT-028]
design: design/exports/v0001-2026-10-25/Settings.dc.html
---

## Description

Show effective tool rules (file edits, shell, network, MCP) with their source, list per-project 'always allow' command grants with revoke, and list configured MCP servers with status.

## Acceptance criteria

- [ ] Per-project grants are listed and revocable
- [ ] Policy-controlled rules show a lock
- [ ] MCP servers show running/stopped/blocked status
- [ ] Revoking a grant makes the next matching command prompt again
