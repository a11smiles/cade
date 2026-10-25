---
id: HIST-013
title: Approve or request changes from the review page
tags: [hist, history, feature, ts]
epic: HIST
priority: P1
depends_on: [GH-015]
design: design/exports/v0001-2026-10-25/Review.dc.html
---

## Description

Approve and Request changes on the review page submit a PR review with that state and the open comments. The page header shows the current review status.

## Acceptance criteria

- [ ] Approve submits an APPROVE review on the PR
- [ ] Request changes submits REQUEST_CHANGES with comments
- [ ] Status pill updates for all viewers on reload
- [ ] Authors cannot approve their own PR (GitHub rule surfaced clearly)
