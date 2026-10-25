---
id: FEED-016
title: Confirm submission and list my feedback
tags: [feed, feedback, feature, ui]
epic: FEED
priority: P1
depends_on: [FEED-007]
design: design/exports/v0001-2026-10-25/Feedback.dc.html
---

## Description

After submit, show the created issue number and link ('Opened #123 · View on GitHub'). Help › My feedback lists issues the user created from CADE with status (open, closed, labels) fetched on demand.

## Acceptance criteria

- [ ] Success state shows issue number and working link
- [ ] My feedback lists in-app issues by this user with current state
- [ ] List refresh is manual or on open, not polling
- [ ] Closed issues show the closing comment summary when available
