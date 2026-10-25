---
id: FOUND-019
title: Add feature flags for unfinished surfaces
tags: [found, chore, rust, ts]
epic: FOUND
priority: P1
depends_on: [FOUND-009]
---

## Description

A small compile-time plus runtime flag system so in-progress output types and importers can ship hidden. Flags can be forced off by org policy.

## Acceptance criteria

- [ ] Flags are declared in one Rust file and mirrored to TS through bindings
- [ ] Hidden surfaces don't appear on Start, menus or the palette
- [ ] Policy can force a flag off; users can't override it
