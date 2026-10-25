---
id: POLICY-022
title: Report installs and policy versions without content
tags: [policy, feature, rust, security]
epic: POLICY
priority: P2
depends_on: [POLICY-015]
design: design/exports/v0001-2026-10-25/Admin.dc.html
---

## Description

When `reporting: on`, send a daily heartbeat (install id, app version, OS, policy version, aggregate token totals by output type) to an org-configured destination: an HTTPS endpoint, or a `workflow_dispatch` on the org's `.github` repo that appends to a private data file. Never send document content, prompts, file names or project names.

## Acceptance criteria

- [ ] Heartbeat payload is exactly the documented fields (schema test)
- [ ] Both destination types work in an integration test
- [ ] Disabling reporting stops heartbeats immediately
- [ ] Admin Overview reads from the destination to fill its cards
