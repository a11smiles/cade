---
id: QA-017
title: Sandbox generated app previews
tags: [qa, feature, security]
epic: QA
priority: P0
depends_on: [QA-016, APP-009]
---

## Description

Generated app code runs only in a separate preview process and webview: its own origin, no Tauri IPC, no access outside the project folder, and network blocked unless policy allows it. Dev servers bind to localhost on random ports, and LAN preview requires an explicit toggle with a one-time token.

## Acceptance criteria

- [ ] Preview webview cannot call any Tauri command (tested)
- [ ] Preview process can't read files outside the project
- [ ] With policy `network: off`, outbound requests from the preview fail
- [ ] LAN preview requires a per-session token in the URL
