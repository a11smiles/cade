---
id: ARCH-008
title: Add typed connections between Azure resources
tags: [arch, feature, rust]
epic: ARCH
priority: P0
depends_on: [ARCH-001, ARCH-007]
design: design/exports/v0001-2026-10-25/Architecture.dc.html
---

## Description

Connections carry meaning, not just lines: network traffic (with port and protocol), private endpoint, managed identity with an RBAC role, secret reference, diagnostic settings and VNet integration. Each type knows the IaC it implies, so drawing a line produces real configuration.

## Acceptance criteria

- [ ] Connection picker offers only types valid for the two endpoints
- [ ] Managed identity connection asks for a role and defaults to least privilege (e.g. Key Vault Secrets User)
- [ ] Private endpoint connection creates the endpoint in a chosen subnet and the private DNS zone link
- [ ] Deleting a connection removes the IaC it generated
- [ ] Connection labels (port, role) render on the line and stay readable at zoom levels down to 25%
