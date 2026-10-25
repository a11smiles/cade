---
id: GH-010
title: Create the pull request with a Copilot-written description
tags: [gh, feature, rust]
epic: GH
priority: P0
depends_on: [GH-009, COPILOT-003]
design: design/exports/v0001-2026-10-25/Ship.dc.html
---

## Description

Push the branch and open a PR (or draft PR) with a title and description drafted by Copilot from the change set, editable before sending, and a link back to the design file.

## Acceptance criteria

- [ ] PR opens with the edited title and description
- [ ] Draft PR option works
- [ ] Description includes pre-flight results and design link
- [ ] Failure to push shows the git error in plain language
