---
id: FEED-018
title: Offer 'Report this' from failure states and crashes
tags: [feed, feedback, feature, ui]
epic: FEED
priority: P1
depends_on: [FEED-009, COPILOT-035, FOUND-015]
design: design/exports/v0001-2026-10-25/States.dc.html
---

## Description

Failure cards (run failed, runtime unavailable) and the crash-recovery dialog get a 'Report this' action that opens the feedback dialog as a Bug prefilled with the error code, failing step and redacted run log reference, still requiring the user to review the preview.

## Acceptance criteria

- [ ] 'Report this' opens a Bug with error details prefilled
- [ ] Run log is attached only if the user ticks the box
- [ ] Crash recovery dialog after restart offers the same action
- [ ] Prefilled text contains no prompt content
