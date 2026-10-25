---
id: FEED-014
title: Suggest similar existing issues before submitting
tags: [feed, feedback, feature, rust, ui]
epic: FEED
priority: P1
depends_on: [FEED-007]
design: design/exports/v0001-2026-10-25/Feedback.dc.html
---

## Description

While the user types a title, search open issues in the CADE repo (search API, debounced) and show up to five likely duplicates with a '+1 / add a comment instead' option that posts a comment or reaction as the user.

## Acceptance criteria

- [ ] Similar issues appear within 1 s of typing pause
- [ ] 'Add my comment' posts to the existing issue instead of creating a new one
- [ ] Search respects API rate limits with caching
- [ ] No search request includes environment info
