---
id: IMAGE-001
title: Create an Image project with size presets
tags: [image, ts, rust, feature]
epic: IMAGE
priority: P0
depends_on: [CORE-003, FOUND-003]
design: design/exports/v0001-2026-10-25/Start.dc.html
---

## Description

Choosing Image on Start creates an Image project with a primary size and suggested companion sizes (LinkedIn 1200×628, Instagram 1080×1080, intranet hero 1920×600, email header 600×200, plus custom). Presets live in a data file the org can extend.

## Acceptance criteria

- [ ] Image projects open in the Image workspace with the primary size
- [ ] Users can add sizes from presets or enter a custom size
- [ ] Orgs can add presets through the design system or policy
