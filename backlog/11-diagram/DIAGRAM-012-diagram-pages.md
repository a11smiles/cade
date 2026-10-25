---
id: DIAGRAM-012
title: Support multiple pages with background pages
tags: [diagram, feature, rust]
epic: DIAGRAM
priority: P1
depends_on: [DIAGRAM-001]
design: design/exports/v0001-2026-10-25/Diagram.dc.html
---

## Description

Page tabs to add, rename, reorder and delete pages; background pages (title blocks, borders) applied to
foreground pages; off-page references that link between pages.

## Acceptance criteria

- [ ] Background pages render beneath assigned foreground pages
- [ ] Off-page references navigate to the target page
- [ ] Pages round-trip with .vsdx including background assignments
