---
id: FEED-002
title: Add the feature request issue form
tags: [feed, feedback, feature]
epic: FEED
priority: P0
depends_on: []
---

## Description

Create `.github/ISSUE_TEMPLATE/feature_request.yml` with: problem, proposed solution, alternatives considered, output type, who benefits (dropdown: individual, team, admin/security), and willingness to contribute. Applies `type:feature` and `needs-triage`.

## Acceptance criteria

- [ ] Form appears on GitHub with required problem and proposal fields
- [ ] Labels `type:feature` and `needs-triage` are applied
- [ ] Field ids match the in-app payload keys
- [ ] Form passes issue-forms schema lint
