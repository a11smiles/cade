---
id: IMPORT-023
title: Route Visio files through the pipeline into Diagram
tags: [import, feature, rust]
epic: IMPORT
priority: P0
depends_on: [IMPORT-001, DIAGRAM-014, DIAGRAM-017]
design: design/exports/v0001-2026-10-25/Import.dc.html
---

## Description

Wire .vsdx, .vsd, .vssx and .vstx into the pipeline: diagrams open in the Diagram workspace, stencils are
added to the project's stencils, and the Visio fidelity report is attached.

## Acceptance criteria

- [ ] Mixed selections of diagrams and stencils import in one batch
- [ ] Stencils appear in the stencil panel after import
- [ ] The Visio fidelity report is shown in the Diagram workspace
