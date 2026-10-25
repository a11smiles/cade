---
id: POLICY-014
title: Strip Visio macros and enforce import rules
tags: [policy, feature, rust, security]
epic: POLICY
priority: P1
depends_on: [POLICY-002, DIAGRAM-018]
design: design/exports/v0001-2026-10-25/Diagram.dc.html
---

## Description

With `outputs.visio_macros: strip` (default), imported `.vsdm` files have VBA projects removed before any parsing beyond the package manifest, and the import report says so. Policy can also block import sources entirely (e.g. disable Figma import).

## Acceptance criteria

- [ ] VBA parts are never extracted or executed (test with a sample .vsdm)
- [ ] Import report lists 'Macros removed'
- [ ] Disabled import sources are hidden in the Import screen
- [ ] Rule is enforced even if UI is bypassed (crate-level check)
