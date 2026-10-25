---
id: SYS-010
title: Guide setting up federated credentials for OIDC
tags: [sys, feature]
epic: SYS
priority: P1
depends_on: [SYS-009]
---

## Description

Generate step-by-step instructions (and optional Bicep for a user-assigned managed identity with federated credentials) to trust the repo's environments, since CADE won't hold credentials to do it itself.

## Acceptance criteria

- [ ] Instructions name the exact subject claims for each environment
- [ ] Optional IaC creates the identity and role assignments
- [ ] Deploy view shows whether OIDC is configured (from a dry-run workflow result)
