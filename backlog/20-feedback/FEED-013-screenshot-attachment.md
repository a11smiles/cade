---
id: FEED-013
title: Spike: attach screenshots to in-app feedback
tags: [feed, feedback, spike]
epic: FEED
priority: P1
depends_on: [FEED-009]
design: design/exports/v0001-2026-10-25/Feedback.dc.html
---

## Description

The GitHub REST API does not support uploading images to issues directly. Evaluate options: (a) capture, let the user crop/blur, copy to clipboard and open the created issue in the browser to paste; (b) upload to a user-owned secret gist and link it; (c) a feedback-assets repo the user can write to. Pick one that keeps the user's identity and doesn't require write access to the CADE repo.

## Acceptance criteria

- [ ] Written comparison of options with privacy and permission implications
- [ ] Prototype of the chosen option works end to end
- [ ] Capture tool supports crop and blur before attaching
- [ ] Decision recorded as an ADR
