---
id: CORE-022
title: Support cross-artifact references between output types
tags: [core, feature, rust]
epic: CORE
priority: P0
depends_on: [CORE-003]
design: design/exports/v0001-2026-10-25/Deploy.dc.html
---

## Description

A project can contain several output types (App, Data model, Architecture, Deck…). Provide a typed reference (`ArtifactRef { page, node_id, kind }`) so an app field can point at a data-model field, a slide can embed a live frame, and an Azure resource can point at the app it runs. References are validated and broken ones are reported.

## Acceptance criteria

- [ ] References survive renames and moves of the target
- [ ] Deleting a referenced node warns and lists dependents
- [ ] Broken references appear in a project problems list
- [ ] Reference graph can be queried by `cade-system`
