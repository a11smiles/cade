---
id: CORE-009
title: Implement components, variants and instances with overrides
tags: [core, feature, rust]
epic: CORE
priority: P0
depends_on: [CORE-003, CORE-007]
design: design/exports/v0001-2026-10-25/Copilot.dc.html
---

## Description

Component nodes define a subtree; variant sets group components by property values (e.g. `state=default|error`, `size=sm|md`). Instance nodes reference a component id and store overrides by stable sub-node path. When the main component changes, instances update except where overridden; overrides survive renames and reorders of unrelated nodes.

## Acceptance criteria

- [ ] Changing a component updates all non-overridden instances
- [ ] Overrides persist through component edits that don't delete the target node
- [ ] Switching an instance's variant keeps compatible overrides
- [ ] Detaching an instance converts it to plain nodes in one undoable step
- [ ] Nested instances (instance inside component) resolve correctly
