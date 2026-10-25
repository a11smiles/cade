---
id: QA-019
title: Fuzz every importer and decoder
tags: [qa, test, security, rust]
epic: QA
priority: P0
depends_on: [QA-016, IMPORT-025]
---

## Description

`cargo-fuzz` targets for .vsdx/.vssx/.vstx XML, legacy .vsd binary, .pptx/.potx, Figma REST JSON, Spark repo manifests, SVG, image decoders, policy.yml and the .cade format itself. Run continuously and apply to OSS-Fuzz once public.

## Acceptance criteria

- [ ] Each importer has a fuzz target with a seed corpus
- [ ] Fuzzers run nightly for at least 1 hour each
- [ ] Crashes create issues automatically with the reproducer
- [ ] Project is submitted to OSS-Fuzz before 1.0

## Notes

Importer-specific corpora and the pptx, Visio and Figma fuzz targets are built in IMPORT-025; this item covers the remaining decoders and continuous fuzzing.
