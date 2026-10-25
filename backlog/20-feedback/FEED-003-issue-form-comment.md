---
id: FEED-003
title: Add the general comment issue form
tags: [feed, feedback, feature]
epic: FEED
priority: P0
depends_on: []
---

## Description

Create `.github/ISSUE_TEMPLATE/comment.yml` for praise, questions and general feedback: topic, comment, optional output type and whether a reply is wanted. Applies `type:comment`. Comments are triaged weekly and may be converted to Discussions.

## Acceptance criteria

- [ ] Form appears on GitHub as 'Comment or question'
- [ ] Label `type:comment` is applied
- [ ] Field ids match the in-app payload keys
- [ ] CONTRIBUTING.md explains when to use each form
