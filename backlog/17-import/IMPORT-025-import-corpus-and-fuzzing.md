---
id: IMPORT-025
title: Build importer test corpora and fuzz targets
tags: [import, test, security]
epic: IMPORT
priority: P1
depends_on: [IMPORT-014, DIAGRAM-014, DECK-021]
design: design/exports/v0001-2026-10-25/Import.dc.html
---

## Description

Licensed corpora for Spark repos, Figma files (as cached API responses), .pptx and Visio files, with expected
reports. Add cargo-fuzz targets for the pptx and visio parsers and the Figma JSON mapper, run nightly.

## Acceptance criteria

- [ ] CI imports every corpus file and compares reports with expectations
- [ ] Fuzz targets run nightly for at least 30 minutes each
- [ ] Any crash found becomes a regression test
