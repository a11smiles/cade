---
id: POLICY-007
title: Enforce model allowlists and the BYOK toggle
tags: [policy, feature, rust]
epic: POLICY
priority: P0
depends_on: [POLICY-002, COPILOT-002]
design: design/exports/v0001-2026-10-25/Admin.dc.html
---

## Description

Filter every model list (Settings, composer picker, fallback) by `models.allow`. When `models.byok` is false, hide BYOK providers and refuse requests to them at the provider layer, not just the UI. `byok_endpoints` restricts OpenAI-compatible hosts.

## Acceptance criteria

- [ ] Disallowed models never appear in pickers
- [ ] A crafted request to a disallowed model is refused in `cade-copilot`
- [ ] BYOK settings show 'Turned off by Contoso policy' with a lock when disabled
- [ ] Changing the allowlist updates pickers without restart
