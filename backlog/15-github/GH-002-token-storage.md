---
id: GH-002
title: Store GitHub tokens in the OS keychain
tags: [gh, feature, rust, security, windows, macos, linux]
epic: GH
priority: P0
depends_on: [GH-001, FOUND-010]
---

## Description

Store tokens in macOS Keychain, Windows Credential Manager and the Secret Service on Linux, never in files or logs. Refresh tokens rotate automatically.

## Acceptance criteria

- [ ] Tokens are stored only in the OS keychain
- [ ] Logs redact tokens
- [ ] Expired tokens refresh silently; failures trigger the sign-in-expired state
- [ ] Linux without a Secret Service shows a clear error and an alternative
