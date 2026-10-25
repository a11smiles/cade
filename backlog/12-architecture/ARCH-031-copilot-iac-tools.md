---
id: ARCH-031
title: Expose architecture operations as Copilot tools
tags: [arch, feature, rust]
epic: ARCH
priority: P0
depends_on: [ARCH-001, ARCH-010, COPILOT-020]
design: design/exports/v0001-2026-10-25/Architecture.dc.html
---

## Description

Register agent tools for adding, moving, connecting and configuring resources, regenerating IaC and running lint, so Copilot edits go through the model and appear as staged changes rather than raw text edits.

## Acceptance criteria

- [ ] Copilot can add a Key Vault with a private endpoint and role assignment in one staged change set
- [ ] Every tool call is validated against the schema and containment rules
- [ ] Staged changes list each resource change and its IaC diff
- [ ] Tools respect policy (blocked services, regions)

## Notes

Uses the COPILOT epic's tool registry and staged change sets.
