---
id: SYS-009
title: Generate a GitHub Actions deploy workflow with OIDC
tags: [sys, feature, rust, security]
epic: SYS
priority: P0
depends_on: [SYS-008]
design: design/exports/v0001-2026-10-25/Deploy.dc.html
---

## Description

Generate `.github/workflows/deploy.yml` that logs in to Azure with OIDC (`azure/login` with federated credentials), deploys IaC, runs migrations, deploys the app to a slot, runs smoke tests and swaps. No secrets are stored in the repo or on the user's machine.

## Acceptance criteria

- [ ] Workflow uses OIDC with `id-token: write` permission and no client secrets
- [ ] Jobs are gated by GitHub environments
- [ ] Workflow passes `actionlint`
- [ ] Regenerating an unchanged plan is byte-identical
