---
id: COPILOT-029
title: Restrict agent file access to the project folder
tags: [copilot, feature, security, rust]
epic: COPILOT
priority: P0
depends_on: [COPILOT-018]
---

## Description

All agent file reads and writes resolve paths canonically (following symlinks) and reject anything outside the open project root, plus `.git/` internals and secrets-looking files (`.env`, `*.pem`, keychains). Writes go through the change set or explicit file-edit staging, never straight to disk.

## Acceptance criteria

- [ ] Path traversal (`../`), absolute paths and symlinks pointing outside are rejected (tests per OS)
- [ ] `.git/` and `.env` reads are denied with a clear tool error
- [ ] File writes by the agent appear as staged file diffs before apply
- [ ] Fuzz test of path inputs finds no escape
