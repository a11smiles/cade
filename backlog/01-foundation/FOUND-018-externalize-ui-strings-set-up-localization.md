---
id: FOUND-018
title: Externalize UI strings and set up localization
tags: [found, feature, ts, ui]
epic: FOUND
priority: P1
depends_on: [FOUND-002]
---

## Description

Use an ICU message format library (e.g. FormatJS) for all chrome strings, with en-US as the source locale. Lay out with logical CSS properties so RTL is possible later. Rust-originated user strings use matching message ids.

## Acceptance criteria

- [ ] No hard-coded user-facing strings remain in components (lint rule enforced)
- [ ] Pseudo-locale build expands strings 40% without clipping key screens
- [ ] Dates, numbers and token counts format per locale
- [ ] RTL pseudo-locale renders panels mirrored without overlap
