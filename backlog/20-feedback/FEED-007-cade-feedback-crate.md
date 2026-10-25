---
id: FEED-007
title: Create cade-feedback crate to submit issues as the user
tags: [feed, feedback, feature, rust]
epic: FEED
priority: P0
depends_on: [GH-017]
design: design/exports/v0001-2026-10-25/Feedback.dc.html
---

## Description

Create `cade-feedback` with a typed payload (kind: bug|feature|comment, fields matching the issue forms, environment, attachments) and a submitter that renders the same Markdown structure as the issue forms and calls `POST /repos/{owner}/{repo}/issues` with the signed-in user's token, so the issue is authored by the user. The target repo is configurable (default: the CADE repo).

## Acceptance criteria

- [ ] Created issue is authored by the signed-in user, not a bot
- [ ] Issue body renders identically to a web-form submission for the same answers
- [ ] Labels `type:*` and `source:in-app` are applied
- [ ] API errors map to user-facing messages (no access, rate limited, validation)
