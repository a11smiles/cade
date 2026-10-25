---
id: COPILOT-009
title: Store provider secrets in the OS keychain
tags: [copilot, feature, rust, security, windows, macos, linux]
epic: COPILOT
priority: P0
depends_on: [FOUND-010]
---

## Description

Create a small secrets module (in `cade-copilot`, reusable by `cade-github` and Azure sign-in) over the `keyring` crate: macOS Keychain, Windows Credential Manager, Linux Secret Service. Secrets are never written to config files, logs, crash reports or the project folder.

## Acceptance criteria

- [ ] Set/get/delete round-trips on all three OSes in CI (Linux uses a headless secret-service test daemon)
- [ ] A grep test over logs and config after a BYOK run finds no secret values
- [ ] If no Secret Service is available on Linux, CADE explains how to install one and refuses to store the key in plain text
- [ ] Removing a provider in Settings deletes its keychain entries
