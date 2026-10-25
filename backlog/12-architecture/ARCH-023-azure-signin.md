---
id: ARCH-023
title: Use the user's own Azure CLI sign-in for read operations
tags: [arch, feature, rust, security]
epic: ARCH
priority: P0
depends_on: []
design: design/exports/v0001-2026-10-25/Settings.dc.html
---

## Description

Read-only Azure operations (what-if, reverse-engineer, listing subscriptions) use the user's existing `az login` session through `AzureCliCredential`. CADE never stores Azure secrets or tokens on disk.

## Acceptance criteria

- [ ] Subscription picker lists subscriptions from the current az session
- [ ] No Azure tokens or secrets are written to disk or logs
- [ ] If not signed in, CADE shows the `az login` command and a retry
- [ ] Tenant switching uses `az account set` semantics
