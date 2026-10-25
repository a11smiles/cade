---
id: FEED-010
title: Preview exactly what will be posted
tags: [feed, feedback, feature, ui, security]
epic: FEED
priority: P0
depends_on: [FEED-009]
design: design/exports/v0001-2026-10-25/Feedback.dc.html
---

## Description

A 'Preview' tab shows the exact issue title, labels and Markdown body (rendered and raw) that will be sent, including environment info and attachments. Submit is only enabled after the preview has been generated for the current content.

## Acceptance criteria

- [ ] Preview matches the created issue byte for byte (test)
- [ ] Raw and rendered views are available
- [ ] Changing any field regenerates the preview
- [ ] Attachments are listed with size and a thumbnail
