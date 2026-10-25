---
id: CORE-010
title: Share components across projects through libraries
tags: [core, feature, rust]
epic: CORE
priority: P1
depends_on: [CORE-009, CORE-008]
---

## Description

Components published in a library can be used in other projects. Instances keep a reference to library id, version and component id. Library updates offer an "update instances" review similar to token upgrades.

## Acceptance criteria

- [ ] Instances from a library show their source and version
- [ ] Update review lists every instance affected
- [ ] Missing library components render a placeholder with the last known snapshot
