---
id: DIAGRAM-016
title: Import legacy binary .vsd files
tags: [diagram, spike, rust]
epic: DIAGRAM
priority: P1
depends_on: [DIAGRAM-014]
design: design/exports/v0001-2026-10-25/Diagram.dc.html
---

## Description

Evaluate options for the pre-2013 binary format: wrap libvisio (MPL-2.0) via FFI behind a feature flag,
or port the needed parts to Rust. Deliver a recommendation and a working import of the reference .vsd files
into the same intermediate model as .vsdx.

## Acceptance criteria

- [ ] Written recommendation covers licensing, platform builds and fidelity
- [ ] Reference .vsd files import with shapes, text and connectors
- [ ] The importer runs in a sandboxed process so a crash cannot take down the app
