---
id: IMPORT-020
title: Report Figma features that cannot be imported
tags: [import, feature, rust]
epic: IMPORT
priority: P0
depends_on: [IMPORT-014, IMPORT-002]
design: design/exports/v0001-2026-10-25/Import.dc.html
---

## Description

Detect plugin data, widgets, FigJam content, unsupported blend modes and effects, and embedded media.
Widgets and plugin-rendered layers come in as locked images; everything is listed in 'Needs attention'.

## Acceptance criteria

- [ ] Widgets and plugin layers import as locked images and are counted
- [ ] Blend-mode flattening is listed per layer
- [ ] The dry run shows these items before the user imports
