---
id: GH-018
title: Handle GitHub rate limits and transient errors
tags: [gh, feature, rust]
epic: GH
priority: P0
depends_on: [GH-001]
---

## Description

Central HTTP client with primary and secondary rate-limit handling, conditional requests (ETags), exponential backoff with jitter and clear user messages when limits are hit.

## Acceptance criteria

- [ ] Secondary rate limits back off using `retry-after`
- [ ] Polling uses ETags to avoid spending quota
- [ ] Users see when a limit resets
- [ ] Retries never duplicate non-idempotent writes
