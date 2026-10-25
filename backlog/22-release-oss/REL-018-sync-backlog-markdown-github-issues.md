---
id: REL-018
title: Sync backlog Markdown to GitHub issues
tags: [rel, feature]
epic: REL
priority: P1
depends_on: []
---

## Description

A script and workflow that create or update one GitHub issue per backlog file (matched by `id`), apply labels from `tags`, epic and priority, set a milestone per priority, and add them to a GitHub Project. The Markdown file stays the source of truth; issue edits don't flow back.

## Acceptance criteria

- [ ] Running the sync twice makes no duplicate issues
- [ ] Labels are created from tags if missing
- [ ] Closing a backlog item (status in file) closes its issue
- [ ] Sync runs on push to `main` when `backlog/` changes
