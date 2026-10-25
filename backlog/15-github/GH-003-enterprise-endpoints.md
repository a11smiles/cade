---
id: GH-003
title: Support GitHub.com, GHE.com and GitHub Enterprise Server
tags: [gh, feature, rust]
epic: GH
priority: P0
depends_on: [GH-001]
design: design/exports/v0001-2026-10-25/Onboarding.dc.html
---

## Description

Let users choose GitHub.com, a GHE.com data-residency tenant or a GHES hostname at sign-in; all API, git and Pages URLs derive from that host.

## Acceptance criteria

- [ ] Host choice appears on first run and in settings
- [ ] API base URLs are derived correctly for each host type
- [ ] GHES version is detected and unsupported features are hidden with an explanation
- [ ] Custom CA certificates from the OS trust store are honored
