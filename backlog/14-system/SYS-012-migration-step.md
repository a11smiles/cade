---
id: SYS-012
title: Run migrations as a deploy step
tags: [sys, feature, rust]
epic: SYS
priority: P0
depends_on: [SYS-009, DATA-014]
design: design/exports/v0001-2026-10-25/Deploy.dc.html
---

## Description

Workflow step applies pending migrations to the environment's database with a short-lived token from OIDC (Entra auth for Azure SQL / PostgreSQL), keeping down scripts available for rollback.

## Acceptance criteria

- [ ] Migrations run before app deployment
- [ ] Database auth uses Entra tokens, not passwords
- [ ] A failed migration stops the workflow before the app deploys
