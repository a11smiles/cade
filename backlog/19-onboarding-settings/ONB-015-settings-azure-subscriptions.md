---
id: ONB-015
title: Connect Azure subscriptions without storing credentials
tags: [onb, feature, rust, security]
epic: ONB
priority: P1
depends_on: [ONB-008, ARCH-023]
design: design/exports/v0001-2026-10-25/Settings.dc.html
---

## Description

Settings › Azure subscriptions signs in with Microsoft Entra ID device code (or reuses an existing `az login` session) to list subscriptions for the Architecture subscription picker and read-only what-if. CADE keeps only refreshable tokens in the OS keychain via MSAL cache encryption — no passwords, no service principal secrets. Deployments never run locally; they go through Actions with OIDC.

## Acceptance criteria

- [ ] Device-code sign-in lists subscriptions the user can read
- [ ] Existing `az` CLI session is detected and offered
- [ ] Tokens are stored only in the OS keychain-backed cache
- [ ] Sign out clears the cache
- [ ] What-if requests use Reader-level calls only
