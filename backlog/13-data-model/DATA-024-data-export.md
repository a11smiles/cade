---
id: DATA-024
title: Export ER diagrams and data dictionaries
tags: [data, feature, rust]
epic: DATA
priority: P2
depends_on: [DATA-004, CANVAS-032]
---

## Description

Export the ER diagram to PNG/SVG/PDF and a data dictionary (Markdown/CSV) listing tables, fields, types, classifications and descriptions.

## Acceptance criteria

- [ ] Data dictionary includes classifications
- [ ] Exports respect the selected dialect
- [ ] Field descriptions entered in the model appear in the dictionary
