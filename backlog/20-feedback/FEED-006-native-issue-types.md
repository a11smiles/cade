---
id: FEED-006
title: Adopt native GitHub issue types if the repo moves to an org
tags: [feed, feedback, chore]
epic: FEED
priority: P2
depends_on: [FEED-005]
---

## Description

GitHub's native issue types are configured at the organization level, so a personal repo uses labels and issue forms instead. If CADE moves to an organization, define Bug, Feature and Comment issue types, set `type:` on each form, migrate existing labels to types, and update the in-app submitter to set the type field.

## Acceptance criteria

- [ ] Decision recorded in an ADR with the trigger (repo transferred to an org)
- [ ] After migration, forms set the native type
- [ ] In-app submissions set the native type via the API
- [ ] Existing issues are migrated by a one-time script
