---
id: COPILOT-020
title: Register output-type tool packs for the agent
tags: [copilot, feature, rust]
epic: COPILOT
priority: P1
depends_on: [COPILOT-019]
---

## Description

Each output type contributes a tool pack registered only in its workspace: App (`generate_code`, `run_tests`, `wire_interaction`), Deck (`add_slide`, `set_chart_data`, `write_notes`), Document (`set_paragraph_style`, `insert_block`), Motion (`add_keyframe`, `set_easing`), Diagram (`connect`, `add_lane`), Architecture (`add_resource`, `set_resource_property`, `run_whatif`), Data model (`add_entity`, `add_field`, `generate_migration`). Packs are defined by their owning epics; this item builds the registry and per-workspace activation.

## Acceptance criteria

- [ ] Registry activates only the current workspace's pack plus core tools
- [ ] A pack can be registered from another crate without changing `cade-copilot`
- [ ] Tool list sent to the model is visible in the context inspector
- [ ] Tool count per request stays under the provider's limit with a test

## Notes

Owning epics (APP, DECK, DOC, MOTION, DIAGRAM, ARCH, DATA) define their tools against this registry.
