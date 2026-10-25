---
id: FOUND-013
title: Provide a network status service
tags: [found, feature, rust]
epic: FOUND
priority: P1
depends_on: [FOUND-004]
design: design/exports/v0001-2026-10-25/States.dc.html
---

## Description

Expose online/offline state and reachability of github.com (or the configured GHES/GHE.com host) to the UI. Used by the offline failure state and the prompt queue. Respect system proxy settings and custom CA bundles common in enterprises.

## Acceptance criteria

- [ ] Going offline updates the UI within 3 s
- [ ] System proxy (including PAC) and OS trust store are honored for all HTTP clients
- [ ] A custom CA bundle path can be set by policy or settings
- [ ] Reachability checks don't send any user data
