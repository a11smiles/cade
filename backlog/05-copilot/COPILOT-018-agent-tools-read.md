---
id: COPILOT-018
title: Expose read-only document tools to the agent
tags: [copilot, feature, rust]
epic: COPILOT
priority: P0
depends_on: [CORE-011, COPILOT-002]
---

## Description

Define CADE's read tools with JSON schemas: `list_pages`, `get_tree(frame)`, `get_layer(id)`, `find_layers(query)`, `get_tokens(design_system)`, `get_component(name)`, `get_selection`, `read_file(path)` (project-scoped), `get_artifact_links`. Outputs are compact and bounded (paged) so large documents don't blow the context window.

## Acceptance criteria

- [ ] Each tool has a JSON schema and a unit test with a sample document
- [ ] Responses over 8k tokens are paged with a cursor
- [ ] `read_file` rejects paths outside the project folder (COPILOT-029)
- [ ] Tools work identically under Copilot and BYOK loops
