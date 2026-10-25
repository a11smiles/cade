---
id: DS-011
title: Build the contextual properties strip
tags: [ds, feature, ui]
epic: DS
priority: P0
depends_on: [DS-008, DS-007]
design: design/exports/v0001-2026-10-25/Image.dc.html
---

## Description

The floating bar at the bottom of the canvas that replaces a fixed inspector. It renders a schema of property groups for the current selection (layer, shape, chart, keyframe, resource, entity, connector, block) and always ends with "Ask Copilot about this". Groups wrap on narrow windows and the strip never covers the selection; it moves above when needed.

## Acceptance criteria

- [ ] Strip content is driven by a property schema per selection type
- [ ] Strip repositions to avoid overlapping the selected element
- [ ] All controls are reachable by keyboard with a shortcut to focus the strip
- [ ] Ask Copilot pre-fills the chat with the selection as a context chip
