---
id: FOUND-010
title: Store secrets in the OS keychain
tags: [found, feature, rust, security]
epic: FOUND
priority: P0
depends_on: [FOUND-001]
---

## Description

Provide a `SecretStore` service over the `keyring` crate: macOS Keychain, Windows Credential Manager and Secret Service (libsecret) on Linux. Used for BYOK API keys, the Figma token and any refresh tokens. Secrets never touch settings files, logs or crash reports.

## Acceptance criteria

- [ ] Set/get/delete works on all three OSes in CI or a documented manual test
- [ ] Linux without a Secret Service shows a clear error and refuses to store in plain text
- [ ] A log-scrubbing test proves secret values never appear in logs
- [ ] Secrets are namespaced per account so two GitHub identities don't collide
