---
id: GH-017
title: Create GitHub issues with the user's identity
tags: [gh, feature, rust]
epic: GH
priority: P0
depends_on: [GH-001]
---

## Description

Client API in `cade-github` to create issues with title, body, labels and issue type (where available) on a given repo using the signed-in user's token; used by the FEED epic.

## Acceptance criteria

- [ ] Issues are created as the signed-in user
- [ ] Labels and issue types are applied when the repo supports them
- [ ] Errors (no access, rate limited) return typed errors
- [ ] Bodies are sent exactly as previewed

## Notes

Consumed by the FEED epic's in-app submission.
