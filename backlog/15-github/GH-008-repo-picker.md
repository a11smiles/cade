---
id: GH-008
title: Choose the target repository and branch for Ship
tags: [gh, feature, ui]
epic: GH
priority: P0
depends_on: [GH-004]
design: design/exports/v0001-2026-10-25/Ship.dc.html
---

## Description

Ship step 1 lists repos the user can push to (searchable), lets them create a new repo, and proposes a branch name. Output choice: code with design-system components, or design spec only.

## Acceptance criteria

- [ ] Repo list is searchable and paged
- [ ] Creating a repo respects org policy on visibility
- [ ] Output choice changes the file list preview
