---
id: ARCH-001
title: Define the architecture document model
tags: [arch, feature, rust]
epic: ARCH
priority: P0
depends_on: [CORE-003]
design: design/exports/v0001-2026-10-25/Architecture.dc.html
---

## Description

Add an architecture document type to `cade-iac` on top of the shared Loro document: resources, containers (subscription, resource group, VNet, subnet), connections and per-resource property bags keyed by Azure resource type and API version. Every node keeps a stable id so canvas, code drawer and Copilot edits all address the same resource.

## Acceptance criteria

- [ ] Resources, containers and connections round-trip through a Loro snapshot without loss
- [ ] Each resource stores type (e.g. `Microsoft.Web/sites`), API version, symbolic name and properties
- [ ] Moving a resource between containers is a single undoable CRDT operation
- [ ] Unit tests cover create, move, delete and concurrent edits from two peers

## Notes

The canvas layer tree and this model share ids; the canvas only renders what this model says exists.
