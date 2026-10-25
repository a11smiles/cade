---
id: FEED-005
title: Define labels as code and sync them
tags: [feed, feedback, chore]
epic: FEED
priority: P0
depends_on: []
---

## Description

Add `.github/labels.yml` with `type:bug`, `type:feature`, `type:comment`, `needs-triage`, `source:in-app`, `severity:*`, and `area:*` labels for each epic/output type, plus a workflow that syncs labels on push to main.

## Acceptance criteria

- [ ] Labels in the repo match `labels.yml` after the workflow runs
- [ ] Removing a label from the file does not delete it from issues without explicit `prune: true`
- [ ] Every output type has an `area:` label
- [ ] Workflow uses the repo's `GITHUB_TOKEN` with least privilege
