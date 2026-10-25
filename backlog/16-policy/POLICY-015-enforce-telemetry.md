---
id: POLICY-015
title: Enforce telemetry and reporting controls
tags: [policy, feature, rust, security]
epic: POLICY
priority: P0
depends_on: [POLICY-002]
design: design/exports/v0001-2026-10-25/Admin.dc.html
---

## Description

Implement `telemetry: off|minimal` and `reporting` switches. CADE ships with telemetry off by default; 'minimal' means crash-free counters only. Designs, prompts, chats and file names are never collected under any setting — this is not configurable.

## Acceptance criteria

- [ ] With telemetry off, no network requests go to any CADE endpoint (traffic test)
- [ ] Reporting sends only install id, app version, OS and policy version when enabled
- [ ] Settings › Privacy shows exactly what is sent
- [ ] There is no code path that sends document content or prompts to a reporting endpoint (code review checklist + test)
