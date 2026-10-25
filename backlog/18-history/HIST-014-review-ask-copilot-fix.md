---
id: HIST-014
title: Send a review comment to Copilot as a fix request
tags: [hist, history, feature, rust, ui]
epic: HIST
priority: P2
depends_on: [GH-015, COPILOT-015]
design: design/exports/v0001-2026-10-25/Review.dc.html
---

## Description

'Ask Copilot to fix in CADE' on a review comment creates a pending prompt in the author's CADE with the comment text and the pinned layer as context. The fix becomes a new version and the review link updates.

## Acceptance criteria

- [ ] Comment becomes a pending prompt with the right selection chip
- [ ] Author must confirm before the run starts
- [ ] Resulting version appears on the review link
- [ ] Comment thread gets a reply linking the new version
