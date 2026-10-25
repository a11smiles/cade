---
id: FOUND-023
title: Package, sign and notarize the macOS build
tags: [found, feature, macos, security]
epic: FOUND
priority: P0
depends_on: [FOUND-021]
---

## Description

Produce a universal (arm64 + x86_64) `.app` in a `.dmg`, signed with a Developer ID certificate, hardened runtime enabled, minimal entitlements, and notarized with `notarytool` and stapled. Secrets for signing live in GitHub environments with required reviewers.

## Acceptance criteria

- [ ] Downloaded DMG opens without Gatekeeper warnings on a clean Mac
- [ ] `codesign --verify --deep --strict` and `spctl -a` pass
- [ ] Entitlements are limited to network client, user-selected files and camera-free defaults
- [ ] Signing runs only from protected release tags
