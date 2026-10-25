---
id: FEED-017
title: Rate-limit feedback submission client-side
tags: [feed, feedback, feature, rust]
epic: FEED
priority: P1
depends_on: [FEED-007]
---

## Description

Limit to 5 submissions per hour per user and debounce double-clicks; respect GitHub secondary rate limits and show retry-after times. Prevents accidental spam and protects the user's API quota.

## Acceptance criteria

- [ ] Sixth submission within an hour is blocked with a retry time
- [ ] Double-clicking Submit creates one issue
- [ ] GitHub 403 secondary-limit responses show the retry-after time
- [ ] Limits are configurable for testing
