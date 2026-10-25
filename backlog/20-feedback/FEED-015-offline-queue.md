---
id: FEED-015
title: Queue feedback when offline and retry
tags: [feed, feedback, feature, rust]
epic: FEED
priority: P1
depends_on: [FEED-007]
design: design/exports/v0001-2026-10-25/Feedback.dc.html
---

## Description

If submission fails due to network, store the submission locally (encrypted at rest with a keychain key) and retry with backoff; show 'Will send when you're back online'. Users can cancel queued feedback.

## Acceptance criteria

- [ ] Offline submit queues the item and confirms
- [ ] Queued items send automatically on reconnect
- [ ] Queued items survive restart
- [ ] Cancel removes the queued item and its data
