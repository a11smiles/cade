---
id: FEED-001
title: Add the bug report issue form
tags: [feed, feedback, feature]
epic: FEED
priority: P0
depends_on: []
---

## Description

Create `.github/ISSUE_TEMPLATE/bug.yml` (GitHub issue form) with: summary, steps to reproduce, expected, actual, output type (dropdown: App, Image, Deck, Document, Motion, Diagram, Architecture, Data model, System, Other), area, severity, environment (app version, OS, GPU, policy version) and optional screenshots. Applies labels `type:bug` and `needs-triage`.

## Acceptance criteria

- [ ] Opening 'New issue › Bug report' on GitHub shows the form with required fields enforced
- [ ] Submitted issues get `type:bug` and `needs-triage` labels
- [ ] Field ids match the in-app payload keys (FEED-007) so in-app and web issues render identically
- [ ] Form validates with GitHub's issue-forms schema (CI lint)
