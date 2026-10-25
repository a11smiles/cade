---
id: ARCH-004
title: Load official Azure Architecture Icons from assets/icons/azure
tags: [arch, feature, rust, ui]
epic: ARCH
priority: P0
depends_on: []
design: design/exports/v0001-2026-10-25/Architecture.dc.html
---

## Description

Read the Microsoft Azure Architecture Icons SVG pack from `assets/icons/azure/` (or a user-chosen folder) at startup and index it by file name and category folder. Icons are drawn unmodified, as the icon terms require. The pack is not committed to the repo; the loader explains how to download it when missing.

## Acceptance criteria

- [ ] Dropping the unzipped pack into `assets/icons/azure/` makes icons appear without code changes
- [ ] Icons render crisply at 16–128 px on the canvas and in the palette
- [ ] SVGs are sanitized (no scripts, external references or embedded fonts) before rendering
- [ ] If the folder is empty, the palette shows a one-time notice with the download link and still works with fallback tiles
- [ ] Icon files are never altered, recolored or cropped
