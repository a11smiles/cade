---
id: CORE-018
title: Write a canonical JSON export for diffs and review
tags: [core, feature, rust]
epic: CORE
priority: P0
depends_on: [CORE-015]
---

## Description

On each save, regenerate `export/project.json`: sorted keys, stable ids, one node per line block, resolved and unresolved token references side by side. This is what humans and PR reviews diff; the CRDT files are the source of truth.

## Acceptance criteria

- [ ] Two saves of an unchanged document produce byte-identical JSON
- [ ] A single property change produces a one-line or few-line diff
- [ ] JSON can be validated against a published JSON Schema
- [ ] Large binary assets are referenced by hash, never inlined
