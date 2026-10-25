---
id: ONB-017
title: Support GHE.com and GHES hosts, sign out and switching
tags: [onb, feature, rust, ui]
epic: ONB
priority: P1
depends_on: [GH-003, ONB-002]
design: design/exports/v0001-2026-10-25/Onboarding.dc.html
---

## Description

On sign-in, let users choose github.com, a GHE.com subdomain, or a GHES hostname. Settings shows the signed-in account with Sign out and Switch account. Signing out clears tokens and policy cache and returns to onboarding without touching project files.

## Acceptance criteria

- [ ] Sign-in works against github.com, GHE.com and a GHES test instance
- [ ] Sign out clears tokens and policy cache
- [ ] Switching accounts reloads policy for the new account
- [ ] Project files are untouched by sign out
