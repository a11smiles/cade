---
id: COPILOT-017
title: Load project instructions and skills
tags: [copilot, feature, rust]
epic: COPILOT
priority: P1
depends_on: [COPILOT-012]
---

## Description

Read `.cade/instructions.md` (and `.github/copilot-instructions.md` if present) as standing project instructions, and load skills from `.cade/skills/<name>/SKILL.md` that Copilot can apply. Show active instructions and skills in the context inspector and allow disabling per thread.

## Acceptance criteria

- [ ] Instructions in `.cade/instructions.md` measurably affect output in a scripted test (e.g. naming convention)
- [ ] `.github/copilot-instructions.md` is picked up when present
- [ ] Skills are listed in the composer's `/` command menu
- [ ] Editing the file updates the next request without restarting
