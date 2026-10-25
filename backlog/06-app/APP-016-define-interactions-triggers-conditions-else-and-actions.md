---
id: APP-016
title: Define interactions: triggers, conditions, else and actions
tags: [app, rust, feature]
epic: APP
priority: P0
depends_on: [APP-015]
design: design/exports/v0001-2026-10-25/Prototype.dc.html
---

## Description

Interactions have a trigger (tap, hover, key, timer, after delay), an optional condition expression (e.g. `step1.isValid`), an else branch, and actions (navigate, open overlay, swap variant, set variable, back, scroll to, open URL).

## Acceptance criteria

- [ ] All listed triggers and actions are available in the interaction strip
- [ ] Conditions reference variables and data-model validation results
- [ ] Else branches route to another frame or state
- [ ] Interactions are stored in the document and survive codegen round trips
