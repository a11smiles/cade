---
id: CORE-003
title: Define the document schema and node kinds
tags: [core, feature, rust]
epic: CORE
priority: P0
depends_on: [CORE-002]
---

## Description

Project → pages → node tree stored in a Loro MovableTree. Node kinds: frame, group, rectangle, ellipse, polygon, star, line, path, text, image, component, instance, connector, slide, chart, table, keyframe track, diagram shape, swimlane, container, Azure resource, entity, field and relationship. Each node has a stable id, kind, name and a property map. Output type limits which kinds can be created on a page.

## Acceptance criteria

- [ ] Schema is documented in `docs/format/schema.md` with every node kind and its properties
- [ ] Creating a disallowed kind on a page fails validation with a clear error
- [ ] Node ids are stable across save, reload, fork and merge
- [ ] Round-trip tests create, save, reload and compare every node kind
