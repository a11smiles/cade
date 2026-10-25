---
id: FOUND-003
title: Boot the Tauri 2 shell with a locked-down capability set
tags: [found, feature, rust, security]
epic: FOUND
priority: P0
depends_on: [FOUND-001, FOUND-002]
---

## Description

Wire `app/src-tauri` as a Tauri 2 app that loads `app/ui`. Start from deny-by-default capabilities: only the IPC commands CADE defines, no shell plugin, no arbitrary filesystem plugin, a strict CSP, and no remote URLs loaded in the main webview. Document each capability file and which window it applies to.

## Acceptance criteria

- [ ] App launches to the Start screen on all three OSes
- [ ] CSP blocks inline scripts and any origin other than the app bundle
- [ ] No Tauri plugin is enabled without an entry in `docs/security/capabilities.md`
- [ ] Attempting a non-allowlisted IPC call from the webview is rejected and logged
- [ ] Release builds disable devtools; debug builds enable them
