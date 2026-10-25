---
id: CORE-029
title: Track project metadata in the manifest
tags: [core, feature, rust]
epic: CORE
priority: P0
depends_on: [CORE-015]
---

## Description

Manifest holds project name, output types present, attached libraries and versions, linked repos and environments, the policy version last applied, default models per output type, and Copilot project instructions path.

## Acceptance criteria

- [ ] Manifest changes are validated against a JSON Schema
- [ ] Policy version is updated whenever a newer policy is applied
- [ ] Manifest is human-readable and diff-friendly
