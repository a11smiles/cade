---
id: GH-020
title: Keep PR state fresh without webhooks
tags: [gh, feature, rust, perf]
epic: GH
priority: P1
depends_on: [GH-018]
---

## Description

Desktop apps can't receive webhooks; poll PR, checks and comments adaptively (fast while a view is open, slow otherwise) within rate limits.

## Acceptance criteria

- [ ] Polling interval adapts to visibility
- [ ] Total requests stay under 10% of the user's hourly limit in normal use
- [ ] Polling pauses while offline and resumes on reconnect
