---
id: IMAGE-011
title: Export every size in one action
tags: [image, rust, feature]
epic: IMAGE
priority: P0
depends_on: [CANVAS-032, IMAGE-003]
design: design/exports/v0001-2026-10-25/Image.dc.html
---

## Description

'Export N sizes' writes every size as PNG, JPEG, WebP or SVG at 1x/2x with a naming pattern (e.g. `open-enrollment_linkedin_1200x628@2x.png`). Optional max file size targets for social platforms compress JPEG/WebP until under the limit.

## Acceptance criteria

- [ ] One action exports all sizes in the chosen formats and scales
- [ ] File names follow the configured pattern
- [ ] File-size targets are met or the export reports which sizes couldn't meet them
- [ ] SVG export keeps text as text unless 'outline text' is chosen
