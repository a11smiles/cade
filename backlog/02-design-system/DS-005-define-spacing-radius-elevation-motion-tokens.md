---
id: DS-005
title: Define spacing, radius, elevation and motion tokens
tags: [ds, feature, ui]
epic: DS
priority: P0
depends_on: [DS-001]
---

## Description

Add a 2/4/6/8/10/12/14/16/20/24/32 px spacing scale, radius tokens (4/6/8/10/12/14/999), two elevation levels for floating strips and dialogs, and motion tokens (`duration.sm/md/lg`, `ease.out/in-out`). Motion tokens collapse to 0 ms under reduced motion.

## Acceptance criteria

- [ ] Components reference only scale tokens for padding, gap and radius
- [ ] Floating property strips and dialogs use the elevation tokens in both themes
- [ ] `prefers-reduced-motion` sets all chrome durations to 0
