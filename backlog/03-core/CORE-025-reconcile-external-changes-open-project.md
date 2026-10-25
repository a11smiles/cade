---
id: CORE-025
title: Reconcile external changes to an open project
tags: [core, feature, rust]
epic: CORE
priority: P1
depends_on: [CORE-020, FOUND-012]
---

## Description

When git pull, a branch switch or another tool changes project files on disk while the project is open, load the new state and merge it with in-memory changes through the CRDT. If the branch changed, prompt before switching the open document.

## Acceptance criteria

- [ ] A `git pull` with new commits updates the open canvas without losing local edits
- [ ] A branch switch outside the app prompts the user
- [ ] Malformed external edits are rejected and the previous state is kept
