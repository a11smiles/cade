---
id: IMAGE-013
title: Add alt text to image exports
tags: [image, ts, feature, a11y]
epic: IMAGE
priority: P1
depends_on: [IMAGE-011]
---

## Description

Each Image project has alt text (Copilot can draft it from the layers). Export writes an alt-text sidecar and embeds it in PNG/JPEG metadata where supported.

## Acceptance criteria

- [ ] Alt text can be written or drafted by Copilot
- [ ] Export writes `<name>.alt.txt` next to each image
- [ ] PNG iTXt / JPEG XMP description is set
