---
id: DIAGRAM-014
title: Read .vsdx files into the diagram model
tags: [diagram, feature, rust]
epic: DIAGRAM
priority: P0
depends_on: [DIAGRAM-001, DIAGRAM-009, DIAGRAM-010]
design: design/exports/v0001-2026-10-25/Diagram.dc.html
---

## Description

`cade-io-visio` reader for the Open Packaging (MS-VSDX) format: package relationships, pages, masters,
shapes and groups, geometry sections to paths, text with character and paragraph formatting, fills, lines,
connects (glue), layers, shape data, themes and background pages. Built-in Visio masters map to CADE masters
when equivalent.

## Acceptance criteria

- [ ] Reference corpus of 20 .vsdx files imports without errors
- [ ] Connector glue is preserved for 100% of glued connectors in the corpus
- [ ] Shape data, layers and pages are preserved
- [ ] Fuzzed inputs never panic or hang (QA fuzz target)
- [ ] A 500-shape file imports in under 3 seconds
