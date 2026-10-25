---
id: ARCH-006
title: Build the service palette with categories and search
tags: [arch, feature, ui, ts]
epic: ARCH
priority: P0
depends_on: [ARCH-005]
design: design/exports/v0001-2026-10-25/Architecture.dc.html
---

## Description

Left panel listing Azure services grouped by category (Compute, Data, Networking, Security & identity, Integration, AI, Monitoring, etc.) with fuzzy search over display name, resource type and common aliases ("app service", "web app", "aks"). Drag a tile onto the canvas to create the resource.

## Acceptance criteria

- [ ] Search returns matches for names, types and aliases in under 50 ms
- [ ] Categories collapse and remember their state per user
- [ ] Recently used and org-approved services appear at the top
- [ ] Keyboard: arrow keys move through results, Enter inserts at the canvas center
- [ ] Services blocked by org policy show a lock and an explanation instead of inserting
