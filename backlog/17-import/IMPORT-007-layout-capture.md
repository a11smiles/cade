---
id: IMPORT-007
title: Capture computed layout by running the app in the preview sandbox
tags: [import, feature, rust, security]
epic: IMPORT
priority: P0
depends_on: [IMPORT-006, APP-009]
design: design/exports/v0001-2026-10-25/Import.dc.html
---

## Description

Run the imported app in the sandboxed preview (no network unless policy allows) and read computed boxes and
styles from the DOM for each screen at mobile and desktop widths, so frames match the real rendering rather
than a guess from JSX.

## Acceptance criteria

- [ ] Imported frames match the running app within 2 px on reference screens
- [ ] Captures both mobile (390 px) and desktop (1280 px) widths
- [ ] The preview runs with network disabled by default
- [ ] Apps that fail to start fall back to JSX-only conversion with a report item
