---
id: IMPORT-002
title: Define the import fidelity report
tags: [import, feature, rust]
epic: IMPORT
priority: P0
depends_on: [IMPORT-001]
design: design/exports/v0001-2026-10-25/Import.dc.html
---

## Description

A structured report produced by every importer: counts per category (native, approximated, flattened to
image or vector, dropped), per-item reasons and element references, and suggested fixes. Saved in the project
as `import-report.json` and shown in the import screen and the target workspace.

## Acceptance criteria

- [ ] Every importer produces a report in the shared schema
- [ ] Report items link to the resulting elements
- [ ] Reports are saved with the project and versioned in git
- [ ] Schema is documented and covered by tests
