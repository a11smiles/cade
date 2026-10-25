---
id: IMAGE-010
title: Scope Copilot edits to the selected shape
tags: [image, rust, feature]
epic: IMAGE
priority: P0
depends_on: [COPILOT-015, IMAGE-003]
design: design/exports/v0001-2026-10-25/Image.dc.html
---

## Description

When a layer is selected, the composer chip (e.g. 'Shape · Sun') scopes Copilot changes to that layer and its effects. The reply lists the property diff (FILL warm-500 → warm-300, ADD glow) and confirms which sizes changed.

## Acceptance criteria

- [ ] Prompts with a selection chip only change the selected layer
- [ ] The reply includes a property diff list
- [ ] Suggested follow-ups are specific to the layer type ('Turn it into a crescent moon')
