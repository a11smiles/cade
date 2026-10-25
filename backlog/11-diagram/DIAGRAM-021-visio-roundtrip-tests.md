---
id: DIAGRAM-021
title: Build a Visio round-trip test corpus
tags: [diagram, test, rust]
epic: DIAGRAM
priority: P1
depends_on: [DIAGRAM-020]
design: design/exports/v0001-2026-10-25/Diagram.dc.html
---

## Description

Corpus of real-world-style .vsdx files (flowcharts, swimlanes, BPMN, org charts, network diagrams) with
expected element counts. CI imports, exports and re-imports each file and compares models.

## Acceptance criteria

- [ ] Round-trip preserves shape count, connector glue, layers and shape data for every corpus file
- [ ] Differences are reported per element in CI output
- [ ] Corpus files are licensed for redistribution
