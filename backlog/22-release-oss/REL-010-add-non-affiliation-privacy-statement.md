---
id: REL-010
title: Add a non-affiliation and privacy statement
tags: [rel, docs]
epic: REL
priority: P0
depends_on: [REL-009]
---

## Description

State in README, About and the docs site that CADE is an independent open-source project not affiliated with or endorsed by GitHub or Microsoft. Publish a privacy page: what leaves the machine (model calls via Copilot or the user's BYOK provider, GitHub API calls for ship/review/feedback), no telemetry by default, and what admins can see.

## Acceptance criteria

- [ ] Statement appears in README, About and the docs footer
- [ ] Privacy page lists every outbound data flow and its purpose
- [ ] Privacy page matches the Admin screen's visibility rules

## Notes

Design reference: design/exports/v0001-2026-10-25/Admin.dc.html
