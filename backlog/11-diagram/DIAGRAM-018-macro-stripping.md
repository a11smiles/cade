---
id: DIAGRAM-018
title: Strip macros from Visio files and never execute them
tags: [diagram, feature, security]
epic: DIAGRAM
priority: P0
depends_on: [DIAGRAM-014]
design: design/exports/v0001-2026-10-25/Diagram.dc.html
---

## Description

.vsdm and .vstm macro-enabled files import with `vbaProject.bin` and ActiveX parts removed. Macros are
never parsed for execution. The import report notes that macros were removed, and org policy can block
macro-enabled files entirely.

## Acceptance criteria

- [ ] Macro parts are absent from the imported project and from re-exported files
- [ ] The import report says how many macro parts were removed
- [ ] Policy `outputs.strip_visio_macros` and an optional block setting are enforced
- [ ] Security test confirms no VBA content is ever passed to an interpreter
