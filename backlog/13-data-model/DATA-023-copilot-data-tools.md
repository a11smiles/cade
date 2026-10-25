---
id: DATA-023
title: Expose data model operations as Copilot tools
tags: [data, feature, rust]
epic: DATA
priority: P0
depends_on: [DATA-001, DATA-014, COPILOT-020]
design: design/exports/v0001-2026-10-25/Data.dc.html
---

## Description

Agent tools to add entities, fields, enums and relationships, classify fields and generate migrations, producing staged change sets with migration previews. Copilot asks domain questions (e.g. can a denied claim be reopened) instead of guessing.

## Acceptance criteria

- [ ] Copilot changes appear as staged with SQL previews
- [ ] Tool calls are validated against the model rules
- [ ] Questions to the user are rendered as answer buttons
- [ ] Generated migrations include down scripts
