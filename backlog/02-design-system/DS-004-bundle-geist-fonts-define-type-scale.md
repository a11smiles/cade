---
id: DS-004
title: Bundle Geist fonts and define the type scale
tags: [ds, feature, ui]
epic: DS
priority: P0
depends_on: [DS-001]
---

## Description

Ship Geist and Geist Mono (SIL OFL) inside the app so no font is fetched at runtime. Define type tokens for chrome: 11/12/13/14/15/22/26/30/40 px steps with weights 400/500/600/700 as used in the screens. Include the OFL license text in the about box and NOTICE.

## Acceptance criteria

- [ ] No network request is made for fonts at runtime
- [ ] Type tokens cover every size used in the v0001 screens
- [ ] OFL license is included in the app bundle and NOTICE file
- [ ] Text renders correctly with OS font scaling at 100%–200%
