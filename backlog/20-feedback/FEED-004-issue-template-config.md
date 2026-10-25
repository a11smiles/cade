---
id: FEED-004
title: Configure issue template chooser and contact links
tags: [feed, feedback, chore]
epic: FEED
priority: P0
depends_on: [FEED-001, FEED-002, FEED-003]
---

## Description

Add `.github/ISSUE_TEMPLATE/config.yml`: disable blank issues, add contact links for security reports (SECURITY.md / private vulnerability reporting) and Discussions for Q&A.

## Acceptance criteria

- [ ] Blank issues are disabled
- [ ] Security link points to private vulnerability reporting, not public issues
- [ ] Chooser shows Bug report, Feature request, Comment or question and the contact links
- [ ] Config passes schema lint
